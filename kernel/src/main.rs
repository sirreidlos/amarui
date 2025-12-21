#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::vec;
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use kernel::task::executor::yield_now;
use kernel::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    task::{Task, executor::Executor, keyboard},
};
use kernel::{logger, println};
// use bootloader::{BootInfo, entry_point};
use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{BootInfo, entry_point};
use core::panic::PanicInfo;
use log::info;
use x86_64::{VirtAddr, structures::paging::Page};

pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use log::error;

    unsafe { logger::LOGGER.get().map(|l| l.force_unlock()) };
    error!("{info}");

    kernel::hlt_loop();
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

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

async fn async_number() -> u32 {
    yield_now().await;
    42
}

async fn example_task() {
    info!("Calling async_number from task 1");
    loop {
        let number = async_number().await;
        // info!("async number task 1: {}", number);
    }
}

async fn example_task2() {
    info!("Calling async_number from task 2");
    loop {
        let number = async_number().await;
        // info!("async number task 2: {}", number);
    }
}

/// this function is the entry point, since the linker looks for a function
/// named `_start` by default
#[unsafe(no_mangle)] // don't mangle the name of this function
pub fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let frame_buffer = match boot_info.framebuffer.take() {
        Some(buffer) => buffer,
        None => panic!("Frame buffer not provided"),
    };

    let frame_info = frame_buffer.info();

    logger::init_logger(
        frame_buffer.into_buffer(),
        frame_info,
        log::LevelFilter::Trace,
    );

    println!("HIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII");

    info!("Hello World{}", "!");
    kernel::init(); // new

    let phys_mem_offset = match boot_info.physical_memory_offset {
        bootloader_api::info::Optional::Some(x) => VirtAddr::new(x),
        bootloader_api::info::Optional::None => panic!("Expected phys mem offset to be Some"),
    };

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(100).write_volatile(0x_f021_f077_f065_f04e) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    info!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    info!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    info!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    info!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(example_task2()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();

    info!("It did not crash!");
    kernel::hlt_loop();
}
