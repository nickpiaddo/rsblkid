// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fs::File;

// From this library

/// Low-level device probe.
#[derive(Debug)]
pub struct Probe {
    pub(crate) inner: libblkid::blkid_probe,
    #[allow(dead_code)]
    file: File,
    #[allow(dead_code)]
    is_read_only: bool,
}

impl Drop for Probe {
    fn drop(&mut self) {
        log::debug!("Probe:: deallocating probe instance");

        unsafe { libblkid::blkid_free_probe(self.inner) }
    }
}
