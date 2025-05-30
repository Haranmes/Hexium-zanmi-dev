use spin::mutex;

pub struct MMIOUtils;

impl MMIOUtils {
    // read
    pub fn read8(address : u64) -> u8{
        unsafe {core::ptr::read_volatile(address as *const u8) }
    }

    pub fn read16(address : u64) -> u16 {
        unsafe { core::ptr::read_volatile(address as *const u16) }
    }

    pub fn read32(address : u64) -> u32 {
        unsafe { core::ptr::read_volatile(address as *const u32) }
    }

    pub fn read64(address : u64) -> u64 {
        unsafe { core::ptr::read_volatile(address as *const u64)}
    }

    // write
    pub fn write8(address : u64, value : u8) {
        unsafe { core::ptr::write_volatile(address as *mut u8, value)}
    }

    pub fn write16(address : u64, value : u16) {
        unsafe { core::ptr::write_volatile(address as *mut u16, value)}
    }

    pub fn write32(address : u64, value : u32) {
        unsafe { core::ptr::write_volatile(address as *mut u64, value)}
    }

    pub fn write64(address : u64, value : u64) {
        unsafe { core::ptr::write_volatile(address as *mut u64, value)}
    }
}