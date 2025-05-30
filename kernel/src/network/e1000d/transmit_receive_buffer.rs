pub const E1000_NUM_RX_DESC : u8 = 32;
pub const E1000_NUM_TX_DESC : u8 = 8;

#[repr(C, packed)]
pub struct E1000RxDesc {
    addr: u64,
    length: u16,
    checksum: u16,
    status: u8,
    errors: u8,
    special: u16,
}

#[repr(C, packed)]
pub struct E1000TXDesc {
    addr: u64,
    length: u16,
    checksum: u16,
    status: u8,
    errors: u8,
    special: u16,
}