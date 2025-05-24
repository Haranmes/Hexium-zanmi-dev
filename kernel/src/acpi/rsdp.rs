use acpi::rsdp::Rsdp;

use crate::boot;
use crate::memory;
use crate::trace;

pub fn get_rsdp() -> &'static Rsdp {
    if let Some(rsdp_res) = boot::RSDP_REQUEST.get_response() {
        let rsdp_ptr = (memory::phys_mem_offset().as_u64() + rsdp_res.address() as u64) as *const Rsdp;
        let rsdp = unsafe {
            &*rsdp_ptr
        };

        // Use `rsdp` safely
        trace!("Signature: {:?}", rsdp.signature());
        trace!("OEM ID: {:?}", rsdp.oem_id());
        trace!("Revision: {}", rsdp.revision());

        rsdp
    } else {
        panic!("Limine did not provide an RSDP response");
    }
}