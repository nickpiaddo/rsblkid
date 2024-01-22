// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// A storage disk.
#[derive(Debug)]
pub struct Disk {
    name: String,
    number: u64,
}

impl Disk {
    pub(super) fn new(name: String, number: u64) -> Disk {
        Self { name, number }
    }

    /// Returns the disk's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the device number associated with the disk.
    pub fn device_number(&self) -> u64 {
        self.number
    }
}
