//! UI related utilities.

use crate::native::{self, string_from_c_str};

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub pid: i32,
    pub title: String,
    pub class_group_name: String,
    pub clsas_instance_name: String,
}

impl WindowInfo {
    pub fn from_active_window() -> WindowInfo {
        unsafe {
            let screen = native::wnck_screen_get_default();
            if screen.is_null() {
                panic!();
            }
            native::wnck_screen_force_update(screen);
            let window = native::wnck_screen_get_active_window(screen);
            if window.is_null() {
                panic!();
            }
            let pid = native::wnck_window_get_pid(window);
            let name = string_from_c_str(native::wnck_window_get_name(window));
            let group_name = string_from_c_str(native::wnck_window_get_class_group_name(window));
            let instance_name =
                string_from_c_str(native::wnck_window_get_class_instance_name(window));

            WindowInfo {
                pid: pid,
                title: name,
                class_group_name: group_name,
                clsas_instance_name: instance_name,
            }
        }
    }
}

#[test]
fn test_from_active_window() {
    gtk::init().unwrap();
    println!("Active window={:?}", WindowInfo::from_active_window());
}
