#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::println;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::{MapperAllSizes,Page},VirtAddr};

    println!("Hello World{}", "!");
    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {os::memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};
    
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    os::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("It did not crash!!");
    os::hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
