#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::{c_void, CStr, CString};

use libc::{c_char, c_int};
include!(concat!(env!("OUT_DIR"), "/native-bindings.rs"));

pub(crate) fn string_from_c_str(s: *const ::std::os::raw::c_char) -> String {
    if s.is_null() {
        panic!("Recived null pointer");
    }
    unsafe { String::from(CStr::from_ptr(s).to_str().expect("String isn't valid UTF-8 sequence")) }
}

pub(crate) fn c_string_from_str(s: &str) -> CString {
    CString::new(s).expect("String contains a NULL character")
}
