use core::fmt;
use crate::errors::{ErrorKind, KernelError};

// TBD: below is hardcoded pl011 serial driver data. Should be moved
//      to BSP as soon as we introduce it.
const PL011_REG_BASE: usize = 0x0900_0000;
const PL011_REG_DR: usize = 0x00;

#[derive(Debug)]
pub struct SerialDevice {}

impl SerialDevice {
    pub fn new() -> Result<Self, KernelError> {
        return Ok(SerialDevice{})
    }

    fn write_byte(&self, b: u8) {
        let dr = (PL011_REG_BASE + PL011_REG_DR) as *mut u8;
        unsafe {
            dr.write_volatile(b);
        }
    }
}

#[derive(Debug)]
pub struct SerialCore {
    device: SerialDevice
}

impl SerialCore {
    pub fn new() -> Result<Self, KernelError> {
        // TBD: this should not be there, but for simplicity let's do it for now.
        let Ok(sd) = SerialDevice::new() else {
            return Err(KernelError::new(ErrorKind::UnknownError, "can't create serial device"));
        };
        
        Ok(SerialCore{
            device: sd,
        })
    }
}

impl fmt::Write for SerialCore {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.bytes() {
            match char {
                b'\n' => {
                    self.device.write_byte(b'\r');
                    self.device.write_byte(b'\n');
                },
                byte => self.device.write_byte(byte),
            }
        }
        Ok(())
    }
}
