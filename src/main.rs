#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dv_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    dv_os::init();

    #[cfg(not(test))]
    {
        use dv_os::{
            allocator, color_code, colored_print, memory, memory::BootInfoFrameAllocator, println,
            Color,
        };
        use x86_64::VirtAddr;

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

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { memory::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");

        let _x = Box::new(69);
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
