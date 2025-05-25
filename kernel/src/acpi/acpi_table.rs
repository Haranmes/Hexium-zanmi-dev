/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use acpi::rsdp::Rsdp;

use crate::{
    acpi::acpi_handler::KernelAcpiHandler, 
    arch::memory, 
    hal::rsdp::RSDP_REQUEST, 
    trace
};

pub fn init_acpi_table() {
    if let Some(rsdp_res) = RSDP_REQUEST.get_response() {
        /* let phys_addr = rsdp_res.address() as u64;
        let virt_addr = memory::phys_mem_offset().as_u64() + phys_addr;
        let rsdp_ptr = virt_addr as *const Rsdp;
        trace!("Rsdp pointer address: {:?}", rsdp_ptr);
        let rsdp = unsafe { &*rsdp_ptr };

        trace!("Signature: {:?}", rsdp.signature());
        trace!("OEM ID: {:?}", rsdp.oem_id());
        trace!("Revision: {}", rsdp.revision()); */
        trace!("Phys mem offset: 0x{:x}", memory::hhdm_offset().as_u64());
        trace!("RSDP physical address: 0x{:x}", rsdp_res.address());

        

        let status = unsafe {
            acpi::AcpiTables::from_rsdp(KernelAcpiHandler {}, rsdp_res.address())
        };

        match status {
            Err(_) => {panic!("Initialization of the Acpi Table was not successfull!")},
            Ok(_) => {trace!("Acpi Table was initialized")}
        }

    } else {
        panic!("Limine did not provide an RSDP response");
    }
}

