#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dv_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialization
    dv_os::init();

    // Initializing heap allocator
    {
        use dv_os::{allocator, memory, memory::BootInfoFrameAllocator};
        use x86_64::VirtAddr;

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { memory::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
        allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");
    }

    // Initialize task executor
    let mut executor = {
        use dv_os::task::{executor::Executor, keyboard, Task};

        let mut executor = Executor::new();
        executor.spawn(Task::new(keyboard::print_keypresses()));
        executor
    };

    // Playing with print macro
    {
        use dv_os::println;

        let number = 123;
        let float = 1.0 / 2.0;
        let boolean = true;
        let emoji = "😭";

        println!("Hello World, I can write evrything!");
        println!(
            "Integer: {}, Float: {}, Boolean: {}, Unknown: {}",
            number, float, boolean, emoji
        );
    }

    // Playing with colored print macro
    {
        use dv_os::{color_code, colored_print, println, Color};

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
    }

    // Playing with interrupts
    {
        use dv_os::{color_code, colored_print, println, Color};

        println!();
        println!("And here comes a breakpoint interrupt...");
        x86_64::instructions::interrupts::int3();
        colored_print!(
            color_code!(Color::Green),
            "Interrupt was handled, and it did not crash!"
        );
        println!();
    }

    // Playing with heap allocator
    {
        use alloc::{boxed::Box, rc::Rc, vec::Vec};
        use dv_os::println;

        let heap_value = Box::new(69);
        println!("heap_value at {:p}", heap_value);

        let mut heap_vector = Vec::new();
        for i in 0..500 {
            if i % 50 == 0 {
                println!(
                    "with {:0>3} values, heap_vector at {:p}",
                    i,
                    heap_vector.as_slice(),
                );
            }
            heap_vector.push(i);
        }
        println!(
            "with 500 values, heap_vector at {:p}",
            heap_vector.as_slice()
        );

        let reference_counted = Rc::new(heap_vector);
        let cloned_reference = reference_counted.clone();
        println!("Ref count is {}", Rc::strong_count(&cloned_reference));
        core::mem::drop(reference_counted);
        println!("New ref count is {}", Rc::strong_count(&cloned_reference));
    }

    // Playing with task executor
    {
        use dv_os::{println, task::Task};

        async fn async_number() -> u32 {
            420
        }

        async fn example_task() {
            let number = async_number().await;
            println!("async number: {}", number);
        }

        executor.spawn(Task::new(example_task()));
    }

    // Running the task executor.
    executor.run();
}

#[cfg(test)]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    dv_os::init();
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
