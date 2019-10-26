#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

pub mod display;
pub mod util;

#[macro_use]
extern crate lazy_static;

use crate::util::qemu::{exit_qemu, QemuExitCode};
use core::panic::PanicInfo;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Typhoos 0.1.0 {}", 2019);
    println!("Hello, World");
    for i in 0..20 {
        println!("line: {}", i);
    }

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial() {
        assert_eq!(1, 1);
    }
}
