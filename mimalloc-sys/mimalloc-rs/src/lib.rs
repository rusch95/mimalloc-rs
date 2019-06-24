use libc_print::println;

#[no_mangle]
pub extern "C" fn print_hello() {
	println!("Hello, World!");
}
