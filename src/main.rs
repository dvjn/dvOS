#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dv_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dv_os::println;

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

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dv_os::test_panic_handler(info)
}
