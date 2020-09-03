#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dv_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    dv_os::init();

    #[cfg(not(test))]
    {
        use dv_os::{pretty_print, println, Color, ColorCode};

        let number = 123;
        let float = 1.0 / 2.0;
        let boolean = true;
        let emoji = "😭";
        let colors = [
            Color::Black,
            Color::Blue,
            Color::Green,
            Color::Cyan,
            Color::Red,
            Color::Magenta,
            Color::Brown,
            Color::LightGray,
            Color::DarkGray,
            Color::LightBlue,
            Color::LightGreen,
            Color::LightCyan,
            Color::LightRed,
            Color::Pink,
            Color::Yellow,
            Color::White,
        ];

        println!("Hello World, I can write.");
        println!();
        println!("I can write number: {} ! ", number);
        println!("I can write float: {} ! ", float);
        println!("I can write boolean: {} ! ", boolean);
        println!("I cannot write emoji: {} ! :(", emoji);
        println!();
        println!("I can wrap a very very very very very very very very very very very very very very very long line!");
        println!();

        println!("I can write in colorful too! :)");
        for fg in colors.iter() {
            for bg in colors.iter() {
                pretty_print!(ColorCode::new(*fg, *bg), " dv ")
            }
            println!();
        }
        println!();

        x86_64::instructions::interrupts::int3();

        pretty_print!(
            ColorCode::new(Color::Green, Color::Black),
            "Interrupt was handled, and it did not crash!"
        );
        println!();

        #[allow(unconditional_recursion)]
        fn stack_overflow() {
            stack_overflow();
            volatile::Volatile::new(0).read();
        }
        stack_overflow();
    }

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use dv_os::println;

    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dv_os::test_panic_handler(info)
}
