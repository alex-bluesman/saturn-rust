#![no_main]
#![no_std]

mod errors;
mod kernel;
mod logger;
mod panic;
mod serial;

#[no_mangle]
extern "C" fn _start_rust_kernel() -> ! {
    kernel::init();
}
