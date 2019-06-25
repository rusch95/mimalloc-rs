/*
#![no_std]

use core::panic::PanicInfo;

use libc;
use libc_print::println;
*/

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    println!("Hello, World!, {}", input);
    input * 2
}

/*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        libc::exit(1)
    }
}
*/
