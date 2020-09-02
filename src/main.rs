#![no_std]
#![no_main]

extern crate rlibc;

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World, I can write.");
    println!();
    println!("I can write number: {} ! ", 123);
    println!("I can write float: {} ! ", 1.0 / 2.0);
    println!("I can write bool: {} ! ", true);
    println!("I cannot write emoji: {} ! :(", "😭");
    println!();
    println!("I can wrap a very very very very very very very very very very very very very very very long line!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked: {}", info);

    loop {}
}
