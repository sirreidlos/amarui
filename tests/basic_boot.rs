#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(amarui::test_runner)]
#![reexport_test_harness_main = "test_main"]

use amarui::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amarui::test_panic_handler(info)
}

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
