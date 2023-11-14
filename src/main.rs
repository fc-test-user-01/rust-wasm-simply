#[no_mangle]
pub extern "C" fn hello_world() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

fn main() {
    println!("Hello, World!");
}
