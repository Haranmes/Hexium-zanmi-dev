use acpi::{AcpiHandler, PhysicalMapping};
use core::ptr::NonNull;

pub fn init_acpi_handler () -> ! {
    #[derive(Copy, Clone)]
    pub struct KernelAcpiHandler {}


    impl AcpiHandler for KernelAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {

        let virt_addr = crate::memory::phys_mem_offset() + physical_address as u64;


        unsafe {
            PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr()).unwrap(), // SAFETY: Memory at virt_addr is mapped by the HHDM and guaranteed to be valid by firmware
            size,
            size,
            self.clone(),
        )
        }
        
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // only called to unload ACPI
    }
    }
}