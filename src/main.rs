#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os::println;
use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os::init();
    
    // let ptr = 0x204920 as *mut u32;
    // unsafe { let x = *ptr; }
    // println!("read worked");
    // unsafe { *ptr = 42; }
    // println!("write worked");

    let (level_4_page_table,_) = Cr3::read();
    println!("Level 4 page table at: {:?}",level_4_page_table.start_address());

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
