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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Writing to the first line of the buffer :) \n").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

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
    loop {}
}