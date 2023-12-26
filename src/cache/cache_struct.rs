// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// Set of information about all block devices on a system.
#[derive(Debug)]
pub struct Cache {
    inner: libblkid::blkid_cache,
}

impl Drop for Cache {
    /// Saves changes to device information into the destination file provided at construction.
    fn drop(&mut self) {
        log::debug!("Cache::drop deallocate `Cache` instance`");

        unsafe { libblkid::blkid_put_cache(self.inner) }
    }
}
