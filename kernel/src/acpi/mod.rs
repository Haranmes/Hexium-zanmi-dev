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

 // Currently only targeting x86
use acpi::{AcpiHandler, PhysicalMapping};
use core::ptr::NonNull;


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

// #[test_case]
// fn acpi_map_physical_region_size_limit() {
//     let handler = KernelAcpiHandler {};

//     // Simulate a physical address (arbitrary for test)
//     let phys_addr = 0x1000usize;

//     // Test with size within HEAP_SIZE - should succeed
//     let size_within = crate::memory::alloc::HEAP_SIZE;
//     let mapping = unsafe { handler.map_physical_region::<u8>(phys_addr, size_within) };
//     assert_eq!(mapping.region_length(), size_within);

//     // Test with size larger than HEAP_SIZE - should panic
//     let size_too_large = crate::memory::alloc::HEAP_SIZE + 1;

//     // TODO: Continue writing test case
// }
