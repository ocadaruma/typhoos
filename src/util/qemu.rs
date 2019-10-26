#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        asm!("outl %eax, %dx" :: "{dx}"(0xf4u16), "{eax}"(exit_code as u32) :: "volatile");
    }
}
