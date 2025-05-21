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

use futures_util::future::ok;
use x86::bits32::paging::Page;
use x86_64::{
    registers::control::Cr3, structures::paging::{
        page_table::FrameError, PageTable, PageTableFlags
    }, PhysAddr, VirtAddr
};

pub fn calculate_mapped_size() -> usize {
    let (level_4_frame, _) = Cr3::read();
    let phys_offset = crate::memory::phys_mem_offset();

    let mut total_mapped_bytes = 0;

    let p4_table_ptr = (phys_offset + level_4_frame.start_address().as_u64()).as_ptr::<PageTable>();
    let p4_table = unsafe { &*p4_table_ptr };

    
    for p4_entry in p4_table.iter() {
        if let Ok(p3_frame) = p4_entry.frame() {
            let p3_table_ptr = (phys_offset + p3_frame.start_address().as_u64()).as_ptr::<PageTable>();
            let p3_table = unsafe { &*p3_table_ptr };

            for p3_entry in p3_table.iter() {
                if let Ok(p2_frame) = p3_entry.frame() {
                    let p2_table_ptr = (phys_offset + p2_frame.start_address().as_u64()).as_ptr::<PageTable>();
                    let p2_table = unsafe { &*p2_table_ptr};

                    for p2_entry in p2_table.iter() {
                        if let Ok(p1_frame) = p2_entry.frame() {
                            let p1_table_ptr = (phys_offset + p1_frame.start_address().as_u64()).as_ptr::<PageTable>();
                            let p1_table = unsafe { &*p1_table_ptr};

                            for p1_entry in p1_table.iter() {
                                if p1_entry.flags().contains(PageTableFlags::PRESENT) {
                                    total_mapped_bytes += 4096;
                                    // Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    total_mapped_bytes     
}



