#![no_std]
#![no_main]

use core::panic::PanicInfo;
use typhoos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Typoos 0.1.0 {}", 2019);
    println!("Hello, World");
    for i in 0..20 {
        println!("line: {}", i);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
