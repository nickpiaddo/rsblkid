// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library
use std::fmt;

// From this library

/// Options for partition search functions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive)]
#[repr(i32)]
pub enum PartitionScanningOption {
    /// Gather details from each partition table entry.
    EntryDetails = libblkid::BLKID_PARTS_ENTRY_DETAILS,
    /// Disable Protective (legacy) MBR detection, which is enabled by default.
    ForceGPT = libblkid::BLKID_PARTS_FORCE_GPT,
    /// Set Magic flag.
    Magic = libblkid::BLKID_PARTS_MAGIC,
}

impl fmt::Display for PartitionScanningOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            PartitionScanningOption::EntryDetails => "Entry Details",
            PartitionScanningOption::ForceGPT => "Force GPT",
            PartitionScanningOption::Magic => "Magic",
        };

        write!(f, "{}", str)
    }
}
