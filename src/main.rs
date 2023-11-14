// src/lib.rs

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
pub extern "C" fn hello_world() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

fn main() {
    println!("");
}
