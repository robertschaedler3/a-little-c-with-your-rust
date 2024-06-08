use rust_plus_c::say_hello;

fn main() {
    let input = std::env::args().nth(1).unwrap_or("Rust".to_string());
    let name = std::ffi::CString::new(input).unwrap();

    unsafe {
        say_hello(name.as_ptr());
    }
}
