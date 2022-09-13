#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(learn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use learn_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("||| Welcome to LearnOS |||");

    // For the many dumbassess like me: Make sure that
    // this is called before anything else, otherwise
    // QEMU will just keep triple faulting
    learn_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initializtion failed!");

    let x = Box::new(4);
    println!("Heap value @ {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("Vec @ {:p}", vec.as_slice());

    let rced = Rc::new(vec![1, 2, 3]);
    let cloned = rced.clone();
    println!("Current refrence count: {}", Rc::strong_count(&cloned));
    core::mem::drop(rced);
    println!("Refrence count is now: {}", Rc::strong_count(&cloned));

    #[cfg(test)]
    test_main();

    learn_os::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    learn_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    learn_os::test_panic_handler(info)
}
