extern crate alloc;

use crate::logger::Logger;
use buddy_system_allocator::LockedHeap;
//use alloc::vec::Vec;

// Saturn heap configuration. We are using buddy allocator, so specify order is enough.
const HEAP_ORDER: usize = 18;
const HEAP_SIZE: usize = 1 << HEAP_ORDER;
#[global_allocator]
static SATURN_HEAP_ALLOCATOR: LockedHeap<HEAP_ORDER> = LockedHeap::<HEAP_ORDER>::new();
static SATURN_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub(crate) fn init() -> ! {
    unsafe {
        SATURN_HEAP_ALLOCATOR.lock().init(SATURN_HEAP.as_ptr() as usize, SATURN_HEAP.len());
    }

    Logger::new().unwrap();

    log::info!("start kernel...");

    loop {}
}
