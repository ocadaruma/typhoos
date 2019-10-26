#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

#[macro_use]
pub mod display;
#[macro_use]
pub mod serial;
pub mod util;

#[macro_use]
extern crate lazy_static;

use crate::util::qemu::{exit_qemu, QemuExitCode};
use core::panic::PanicInfo;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial() {
        serial_print!("trivial assertion... ");
        assert_eq!(1, 1);
        serial_println!("[ok]");
    }
}
