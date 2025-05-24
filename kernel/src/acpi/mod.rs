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

use acpi_handler::init_acpi_handler;


pub mod rsdp;
pub mod acpi_handler;

pub fn init() {
    init_acpi_handler();
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
