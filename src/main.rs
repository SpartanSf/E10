
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(e10::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use e10::println;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use e10::memory;
    use x86_64::{structures::paging::Page, VirtAddr}; // new import
    //use x86_64::{structures::paging::Translate, VirtAddr};


    
    println!("Hello World{}", "!");
    e10::init();



    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};



    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    e10::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    e10::hlt_loop();            // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    e10::test_panic_handler(info)
}