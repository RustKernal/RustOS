#![no_std]
#![no_main]
use kernal::println;
use kernal::spin;
use bootloader::{entry_point,BootInfo};

entry_point!(main);

fn main(bootInfo: &'static BootInfo) -> ! {
    kernal::init();
    kernal::init_heap(bootInfo);
    kernal::println!("Hello World!");
    spin!()
}