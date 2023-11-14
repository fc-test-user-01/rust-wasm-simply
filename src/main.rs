use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;

// Define a global variable to store a value
static mut MY_VALUE: i32 = 0;

#[no_mangle]
pub extern "C" fn set_value(value: i32) {
    // Use unsafe to modify the global variable
    unsafe {
        MY_VALUE = value;
    }
}

#[no_mangle]
pub extern "C" fn get_value() -> i32 {
    // Use unsafe to read the global variable
    unsafe {
        MY_VALUE
    }
}

#[no_mangle]
pub extern "C" fn get_string(ptr: *mut c_char) -> *const c_char {
    if ptr.is_null() {
        return ptr::null();
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };
    let rust_string = format!("Rust says: {}", c_str.to_string_lossy());
    let new_c_string = CString::new(rust_string).expect("Failed to create CString");
    new_c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn hello_world() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

fn main() {
    println!("");
}
