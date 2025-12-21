use crate::{framebuffer::FrameBufferWriter, serial_println};
use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use core::fmt::{self, Write};
use log::LevelFilter;
use spin::Mutex;
use x86_64::instructions::interrupts;

/// The global logger instance used for the `log` crate.
pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

/// A logger instance protected by a spinlock.
pub struct LockedLogger {
    framebuffer: Option<Mutex<FrameBufferWriter>>,
}

impl LockedLogger {
    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let framebuffer = Mutex::new(FrameBufferWriter::new(framebuffer, info));

        LockedLogger {
            framebuffer: Some(framebuffer),
        }
    }

    /// Force-unlocks the logger to prevent a deadlock.
    ///
    /// ## Safety
    /// This method is not memory safe and should be only used when absolutely necessary.
    pub unsafe fn force_unlock(&self) {
        if let Some(framebuffer) = &self.framebuffer {
            unsafe { framebuffer.force_unlock() };
        }
    }
}

impl log::Log for LockedLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        interrupts::without_interrupts(|| {
            if let Some(framebuffer) = &self.framebuffer {
                let mut framebuffer = framebuffer.lock();
                writeln!(framebuffer, "{:5}: {}", record.level(), record.args()).unwrap();
            }

            serial_println!("{:5}: {}", record.level(), record.args());
        })
    }

    fn flush(&self) {}
}

fn convert_level(level: LevelFilter) -> log::LevelFilter {
    match level {
        LevelFilter::Off => log::LevelFilter::Off,
        LevelFilter::Error => log::LevelFilter::Error,
        LevelFilter::Warn => log::LevelFilter::Warn,
        LevelFilter::Info => log::LevelFilter::Info,
        LevelFilter::Debug => log::LevelFilter::Debug,
        LevelFilter::Trace => log::LevelFilter::Trace,
    }
}

/// Initialize a text-based logger using the given pixel-based framebuffer as output.
pub fn init_logger(framebuffer: &'static mut [u8], info: FrameBufferInfo, log_level: LevelFilter) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(framebuffer, info));
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(convert_level(log_level));
    log::info!("Framebuffer info: {:?}", info);
}

impl LockedLogger {
    fn print(&self, args: core::fmt::Arguments) {
        use core::fmt::Write;

        interrupts::without_interrupts(|| {
            if let Some(framebuffer) = &self.framebuffer {
                let mut fb = framebuffer.lock();
                fb.write_fmt(args).unwrap();
            }
            serial_println!("{}", args);
        });
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logger::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        LOGGER.get().unwrap().print(args);
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 6][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
