//! UI related utilities.
// Ported from https://github.com/UltimateHackingKeyboard/current-window-linux/blob/master/get-current-window.c
// Use xprop(1) to list all properties.

use libc::{c_char, c_int, c_uchar, c_uint, c_ulong};
use std::{error::Error, ptr};
use x11::xlib::{self, Display};

use crate::native::{c_string_from_str, string_from_c_str};

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
    pub pid: u64,
    pub title: String,
    pub class_group_name: String,
    pub clsas_instance_name: String,
}

impl WindowInfo {
    pub fn from_active_window() -> anyhow::Result<WindowInfo> {
        return get_active_window_info();
    }
}

#[test]
fn test_from_active_window() {
    // x_init_threads();
    println!("Active window={:?}", WindowInfo::from_active_window().unwrap());
}

unsafe fn get_property(display: *mut Display, window: c_ulong, filter: &str) -> anyhow::Result<*const c_uchar> {
    let filter_atom = xlib::XInternAtom(display, c_string_from_str(filter).as_ptr(), 1);

    let mut actual_type: c_ulong = 0;
    let mut actual_format: c_int = 0;
    let mut ntimes: c_ulong = 0;
    let mut bytes_after: c_ulong = 0;
    let mut prop: *mut c_uchar = ptr::null_mut();

    let status = xlib::XGetWindowProperty(
        display,
        window,
        filter_atom,
        0,
        1024,
        0, // False
        xlib::AnyPropertyType as u64,
        &mut actual_type,
        &mut actual_format,
        &mut ntimes,
        &mut bytes_after,
        &mut prop,
    );
    if status != xlib::Success as i32 {
        anyhow::bail!("XGetWindowProperty() failed: status={}", status);
    }
    log::debug!(
        "XGetWindowProperty({}) returned Success. type={} format={} prop={:?}",
        filter,
        actual_type,
        actual_format,
        prop
    );
    if prop.is_null() {
        anyhow::bail!("XGetWindowProperty() returned null");
    }

    return Ok(prop);
}

unsafe fn get_long_property(display: *mut Display, window: c_ulong, filter: &str) -> anyhow::Result<u64> {
    let res = get_property(display, window, filter)?;
    return Ok((*res as u64) + ((*res.offset(1) as u64) << 8) + ((*res.offset(2) as u64) << 16) + ((*res.offset(3) as u64) << 24));
}

unsafe fn get_string_property(display: *mut Display, window: c_ulong, filter: &str) -> anyhow::Result<String> {
    let res = get_property(display, window, filter)?;
    return Ok(string_from_c_str(res as *const c_char));
}

unsafe fn get_double_string_property(display: *mut Display, window: c_ulong, filter: &str) -> anyhow::Result<(String, String)> {
    let res = get_property(display, window, filter)?;

    // The result contains two consecutive c-strings.

    let first = string_from_c_str(res as *const c_char);

    let mut i = 0;
    loop {
        if *res.offset(i) == 0 {
            break;
        }
        i += 1;
    }
    let second = string_from_c_str(res.offset(i + 1) as *const c_char);

    return Ok((first, second));
}

fn get_active_window_info() -> anyhow::Result<WindowInfo> {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display == ptr::null_mut() {
            anyhow::bail!("XOpenDisplay() failed. (Is it under X11?)");
        }
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);

        let active = get_long_property(display, root, "_NET_ACTIVE_WINDOW")?;
        let pid = get_long_property(display, active, "_NET_WM_PID")?;
        let title = get_string_property(display, active, "_NET_WM_NAME")?;
        let class = get_double_string_property(display, active, "WM_CLASS")?;

        return Ok(WindowInfo {
            pid,
            title,
            class_group_name: class.1,
            clsas_instance_name: class.0,
        });
    }
}
