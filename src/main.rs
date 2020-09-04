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
        use dv_os::{color_code, colored_print, println, Color};

        let number = 123;
        let float = 1.0 / 2.0;
        let boolean = true;
        let emoji = "😭";

        println!("Hello World, I can write evrything!");
        println!(
            "Integer: {}, Float: {}, Boolean: {}, Unknown: {}",
            number, float, boolean, emoji
        );

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

        println!("I can write in colorful too! :)");
        for fg in colors.iter() {
            for bg in colors.iter() {
                colored_print!(color_code!(*fg, *bg), " dv ")
            }
            println!();
        }
        println!();

        println!("And here comes a breakpoint interrupt...");
        x86_64::instructions::interrupts::int3();
        colored_print!(
            color_code!(Color::Green),
            "Interrupt was handled, and it did not crash!"
        );
        println!();
    }

    #[cfg(test)]
    test_main();

    dv_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use dv_os::println;

    println!("{}", info);
    dv_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dv_os::test_panic_handler(info)
}
