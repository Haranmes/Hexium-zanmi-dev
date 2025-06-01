use core::arch::asm;

#[repr(C)]
pub struct Ports;

impl Ports {
    #[cfg(target_arch = "x86_64")]
    pub fn outb(port: u16, data: u8) {
        unsafe {
            asm!(
                "outb %al, %dx",
                in("dx") port,
                in("al") data,
                options(nostack, nomem, preserves_flags),
            );
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn outw(port: u16, data: u16) {
        unsafe {
            asm!(
                "outw %ax, %dx",
                in("dx") port,
                in("ax") data,
                options(nostack, nomem, preserves_flags),
            );
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn inb(port: u16) -> u8 {
        let mut ret: u8;
        unsafe {
            asm!(
                "inb %dx, %al",
                in("dx") port,
                out("al") ret,
                options(nostack, nomem, preserves_flags),
            );
        }
        ret
    }

    #[cfg(target_arch = "x86_64")]
    pub fn inw(port: u16) -> u16 {
        let mut ret: u16;
        unsafe {
            asm!(
                "inw %dx, %ax",
                in("dx") port,
                out("ax") ret,
                options(nostack, nomem, preserves_flags),
            );
        }
        ret
    }

    #[cfg(target_arch = "x86_64")]
    pub fn inl(port: u16) -> u32 {
        let mut ret: u32;
        unsafe {
            asm!(
                "inl %dx, %eax",
                in("dx") port,
                out("eax") ret,
                options(nostack, nomem, preserves_flags),
            );
        }
        ret
    }
}
