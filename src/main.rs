// main.rs

// Unlinking the standard library
#![no_std]
// Ovewriting the main application entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
/* the test harness works by redefining the main function
but there isnt one so we need to point it to a new a function name
which can be called in _start()
*/
#![reexport_test_harness_main = "test_main"]

// Modules
mod vga_buffer;

// Disable name mangling so the compiler doesnt generate a unique name
#[no_mangle]
// This is the overidden entry point
pub extern "C" fn _start() -> ! {
    // Call the test harness
    #[cfg(test)]
    test_main();
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

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}