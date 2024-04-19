#![cfg_attr(not(feature = "std"), no_std)]
#![no_main] // disable all Rust-level entry points

extern crate lazy_static;
extern crate volatile;
extern crate spin;

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("I'm definitely not addicted to crack :)");
    panic!("Whoops!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}