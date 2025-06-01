#[repr(C)]
pub struct NetworkDriver {
    bar_type : u8,          // Type of BAR0
    io_base : u16,          // IO Base Address
    mem_base : u64,         // MMIO Base Address
    eerprom_exists : bool,  // A flag indicating if eeprom exists
    mac: [u8; 6],           // A buffer for storing the mack address
    rx_cur : u16,           // Current Receive Descriptor Buffer
    tx_cur : u16,           // Current Transmit Descriptor Buffer
}

pub trait NetworkDriverTrait {
    // Send Commands and read results From NICs either using MMIO or IO Ports
    fn write_command (&mut self, address : u16, value : u32);
    fn read_command (&self, address : u16) -> u32;

    fn detect_eeprom(&mut self) -> bool;        // Return true if EEProm exist, else it returns false and set the eerprom_existsdata member
    fn eeprom_read(&self, addr : u8) -> u32;    // Read 4 bytes from a specific EEProm Address
    fn read_mac_address(&self) -> bool;         // Read MAC Address
    fn start_link(&mut self);                   // Start up the network
    fn rxinit(&mut self);                       // Initialize receive descriptors an buffers
    fn txinit(&mut self);                       // Initialize transmit descriptors an buffers
    fn enable_interrupt(&mut self);             // Enable Interrupts
    fn handle_receive(&mut self);               // Handle a packet reception.

    // public
    fn start(&mut self);                        // perform initialization tasks and starts the driver
    fn fire(&mut self);                         // TODO: Add Interrupt Context
}

