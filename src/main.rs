#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(yan_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use yan_os::task::executor::Executor;
use core::panic::PanicInfo;
use yan_os::println;
use yan_os::task::keyboard;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use yan_os::task::{Task, simple_executor::SimpleExecutor};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    use yan_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::{VirtAddr, structures::paging::Page};
    use yan_os::allocator; // new import

    println!("Hello World{}", "!");
    yan_os::init(); // new    
    
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // let mut frame_allocator = unsafe {
    //     BootInfoFrameAllocator::init(&boot_info.memory_map)
    // };

    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    // // create a dynamically sized vector
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    // core::mem::drop(reference_counted);
    // println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::pritn_keypresses()));
    // executor.run();

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    yan_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    yan_os::hlt_loop();            // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yan_os::test_panic_handler(info)
}