use crate::errors::KernelError;
use crate::serial::SerialCore;

use conquer_once::spin::OnceCell;
use core::fmt::Write;
use spinning_top::Spinlock;

pub static LOGGER: OnceCell<Logger> = OnceCell::uninit();

pub struct Logger {
    serial: Spinlock<SerialCore>,
}

impl Logger {
    pub fn new() -> Result<(), KernelError> {
        let core = match SerialCore::new() {
            Ok(serial_core) => serial_core,
            Err(err) => return Err(err),
        };

        let logger = LOGGER.get_or_init(move || {
            Logger {
                serial: Spinlock::new(core),
            }
        });

        log::set_logger(logger).expect("logger already set"); 
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut serial = self.serial.lock();
        writeln!(serial, "{:5}: {}", record.level(), record.args()).unwrap()
    }

    fn flush(&self) {}
}
