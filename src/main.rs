#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(amarui::test_runner)]
#![reexport_test_harness_main = "test_main"]

use amarui::{
    QemuExitCode, exit_qemu, print, println, serial_print, serial_println, vga_buffer::WRITER,
};
use core::panic::PanicInfo;
use spin::Mutex;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// this function is the entry point, since the linker looks for a function
/// named `_start` by default
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // panic!("Some panic message");
    println!("Hello world{}", "!");

    let s = "Some test string that fits on a single line";
    println!("{}", s);

    let mut curr_buffer: [[u8; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[0; BUFFER_WIDTH]; BUFFER_HEIGHT];
    for r in 0..BUFFER_HEIGHT {
        for c in 0..BUFFER_WIDTH {
            let screen_char = WRITER.lock().buffer.chars[r][c].read().ascii_character;
            curr_buffer[r][c] = screen_char
        }
    }

    for r in 0..BUFFER_HEIGHT {
        for c in 0..BUFFER_WIDTH {
            print!("{}", char::from(curr_buffer[r][c]));
        }

        println!();
    }
    println!("ASJDJAJKSD");

    #[cfg(test)]
    test_main();

    loop {}
}
