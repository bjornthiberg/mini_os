#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle function name
pub extern "C" fn _start() -> ! {
    // linker looks for a function named `_start` by default
    loop {}
}

/// called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
