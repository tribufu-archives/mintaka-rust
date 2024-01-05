// Copyright (c) Tribufu. All Rights Reserved.

use std::ffi::{CStr, CString};

pub use libc::c_char;
pub use libc::c_void;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub trait FString
where
    Self: Sized,
{
    fn to_char_pt(self) -> *const c_char;
    fn from_char_pt(pt: *const c_char) -> String;
}

impl FString for String {
    fn to_char_pt(self) -> *const c_char {
        CString::new(self).unwrap().into_raw()
    }

    fn from_char_pt(pt: *const c_char) -> String {
        let contents = unsafe { CStr::from_ptr(pt) };
        contents.to_str().unwrap().to_string()
    }
}

/// Convert Rust string to C string.
pub fn from_string(string: String) -> *const c_char {
    string.to_char_pt()
}

/// Convert C string to Rust string.
pub fn to_string(pt: *const c_char) -> String {
    String::from_char_pt(pt)
}
