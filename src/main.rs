#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use typhoos::display::vga_buffer::Writer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();

    writeln!(writer, "Typoos 0.1.0 {}", 2019).unwrap();
    writeln!(writer, "Hello, World").unwrap();
    for i in 0..20 {
        writeln!(writer, "line: {}", i).unwrap();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
