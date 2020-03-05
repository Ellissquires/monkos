// main.rs

// Unlinking the standard library
#![no_std]
// Ovewriting the main application entry point
#![no_main]

mod vga_buffer;


// Disable name mangling so the compiler doesnt generate a unique name
#[no_mangle]
// This is the overidden entry point
pub extern "C" fn _start() -> ! {

    println!("Hello");
    println!("Println now works!");

    panic!("Some panic message");

    loop {}

}

use core::panic::PanicInfo;

/* Panic Handler
The Panic handler is called when Unreoverable Errors occur,
The function should never return so it is marked as a divergent
function by returning the "never" type !.
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}