#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernal::{println, serial_println, serial_print, spin};
use kernal::{clear};



use bootloader::entry_point;
use bootloader::BootInfo;

extern crate alloc;

use alloc::boxed::Box;

entry_point!(main);

#[no_mangle]
fn main(boot_info: &'static BootInfo) -> ! {
    kernal::init();
    kernal::init_heap(boot_info);
    #[cfg(test)]
    test_main();

    

    clear!();
    println!("Hello World!");

    let x= Box::new(90);
    println!("{}", *x);

    println!("Still Here!");
    spin!();
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}



#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}