//! UI related utilities.

use core::fmt;
use std::error::Error;

use crate::native;

#[derive(Debug)]
pub enum GuiError {
    UnknownError(String),
}

impl GuiError {
    pub fn new_unknown_error(message: String) -> GuiError {
        return GuiError::UnknownError(message);
    }
}

impl Error for GuiError {}

impl fmt::Display for GuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self {
            &GuiError::UnknownError(inner) => write!(f, "Unknown error: {}", inner),
        };
    }
}


// Note, looks like this doesn't need to be called on the I/O thread to use `get_active_window_info()`.
pub fn x_init_threads() {
    unsafe {
        if crate::native::XInitThreads() == 0 {
            panic!("XInitThreads() returned 0");
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowInfo {
    pub title: String,
    pub class_group_name: String,
    pub clsas_instance_name: String,
}

impl WindowInfo {
    pub fn from_active_window() -> Result<WindowInfo, Box<dyn Error>> {
        // return get_active_window_info();
        todo!();
    }
}

const MAX_STR: i64 = 256;

#[test]
fn test_from_active_window() {
    // x_init_threads();
    println!("Active window={:?}", WindowInfo::from_active_window());
}

unsafe fn get_string_property(display: *mut native::Display, window: libc::c_ulong, property_name: String) -> String {
    let mut actual_type: native::Atom;
    let mut actual_format: libc::c_int;
    let mut ntimes: libc::c_int;
    let mut bytes_after: libc::c_int;
    let mut result: *mut libc::c_char;
    let filter = native::XInternAtom(display, native::c_string_from_str(&property_name).as_ptr(), 1);

    let status = native::XGetWindowProperty(display, window, filter, 0, MAX_STR, 0, native::AnyPropertyType, &mut actual_type, &actual_format,&ntimes, &bytes_after,  &result);
    match status {
        native::Success => {
            // ok
        }
        _ => {
            panic!("XGetWindowProperty() failed with {}", status);
        }
    }
}

fn get_active_window_info() -> Result<WindowInfo, Box<dyn Error>> {
    unsafe {
        let display = native::XOpenDisplay(std::ptr::null());
        if display.is_null() {
            return Err(Box::new(GuiError::UnknownError("XOpenDisplay() failed.".to_string())));
        }
        let screen = native::XDefaultScreen(display);
        let window = native::xRootWindow(display, screen);
    }
    todo!();
}

// fn get_active_window_info() -> Result<WindowInfo, Box<dyn Error>> {
//     // Set up our state
//     let (conn, screen) = XCBConnection::connect(None)?;
//     let root = conn.setup().roots[screen].root;
//     let mut net_active_window = LazyAtom::new(&conn, false, b"_NET_ACTIVE_WINDOW");
//     let mut net_wm_name = LazyAtom::new(&conn, false, b"_NET_WM_NAME");
//     let mut utf8_string = LazyAtom::new(&conn, false, b"UTF8_STRING");

//     let focus = find_active_window(&conn, root, net_active_window.atom()?)?;

//     // Collect the replies to the atoms
//     let (net_wm_name, utf8_string) = (net_wm_name.atom()?, utf8_string.atom()?);
//     let (wm_class, string) = (Atom::WM_CLASS.into(), Atom::STRING.into());

//     // Get the property from the window that we need
//     let name = conn.get_property(false, focus, net_wm_name, utf8_string, 0, u32::max_value())?;
//     let class = conn.get_property(false, focus, wm_class, string, 0, u32::max_value())?;
//     let (name, class) = (name.reply()?, class.reply()?);

//     // Print out the result
//     let (instance, class) = parse_wm_class(&class);

//     // println!("Window name: {:?}", parse_string_property(&name));
//     // println!("Window instance: {:?}", instance);
//     // println!("Window class: {:?}", class);

//     return Ok(WindowInfo {
//         title: parse_string_property(&name).to_string(),
//         class_group_name: class.to_string(),
//         clsas_instance_name: instance.to_string(),
//     });
// }

// fn find_active_window(conn: &impl Connection, root: WINDOW, net_active_window: ATOM) -> Result<WINDOW, Box<dyn Error>> {
//     let window = Atom::WINDOW.into();
//     let active_window = conn.get_property(false, root, net_active_window, window, 0, 1)?.reply()?;
//     if active_window.format == 32 && active_window.length == 1 {
//         // Things will be so much easier with the next release:
//         // This does active_window.value32().next().unwrap()
//         Ok(u32::try_parse(&active_window.value)?.0)
//     } else {
//         // Query the input focus
//         Ok(conn.get_input_focus()?.reply()?.focus)
//     }
// }

// fn parse_string_property(property: &GetPropertyReply) -> &str {
//     std::str::from_utf8(&property.value).unwrap_or("Invalid utf8")
// }

// fn parse_wm_class(property: &GetPropertyReply) -> (&str, &str) {
//     if property.format != 8 {
//         panic!("Malformed property: wrong format");
//         // return ("Malformed property: wrong format", "Malformed property: wrong format");
//     }
//     let value = &property.value;
//     // The property should contain two null-terminated strings. Find them.
//     if let Some(middle) = value.iter().position(|&b| b == 0) {
//         let (instance, class) = value.split_at(middle);
//         // Skip the null byte at the beginning
//         let mut class = &class[1..];
//         // Remove the last null byte from the class, if it is there.
//         if class.last() == Some(&0) {
//             class = &class[..class.len() - 1];
//         }
//         let instance = std::str::from_utf8(instance);
//         let class = std::str::from_utf8(class);
//         (instance.unwrap_or("Invalid utf8"), class.unwrap_or("Invalid utf8"))
//     } else {
//         panic!("Missing null byte");
//         // ("Missing null byte", "Missing null byte")
//     }
// }
