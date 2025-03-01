#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

#[allow(non_camel_case_types, non_upper_case_globals, unused, non_snake_case)]
pub mod ffi;

pub mod fmt;

static CNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn rust_main() -> u32 {
    let cnt = CNT.fetch_add(1, Ordering::SeqCst);
    println!("Hello, Rust {}!!!", cnt);
    cnt
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC:\n{}", _info);
    loop {}
}
