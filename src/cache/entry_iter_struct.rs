// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;
use std::ptr::NonNull;

// From this library
use crate::cache::Cache;
use crate::cache::Device;
use crate::cache::EntryIterError;

/// Iterator over a collection of [`Device`]s.
#[derive(Debug)]
pub struct EntryIter<'a> {
    inner: libblkid::blkid_dev_iterate,
    cache: &'a Cache,
}

impl<'a> EntryIter<'a> {
    /// Creates an `EntryIter`.
    pub(super) fn new(cache: &'a Cache) -> Result<EntryIter<'a>, EntryIterError> {
        log::debug!("EntryIter::new creating a new `EntryIter` instance");

        let mut iterator = MaybeUninit::<libblkid::blkid_dev_iterate>::zeroed();
        unsafe {
            iterator.write(libblkid::blkid_dev_iterate_begin(cache.inner));
        };

        match unsafe { iterator.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `EntryIter` instance".to_owned();
                log::debug!(
                    "EntryIter::new {}. libblkid::blkid_dev_iterate_begin returned a NULL pointer",
                    err_msg
                );

                Err(EntryIterError::Creation(err_msg))
            }
            inner => {
                log::debug!("EntryIter::new created a new `EntryIter` instance");
                let inner = Self { inner, cache };

                Ok(inner)
            }
        }
    }
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = Device<'a>;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("EntryIter::next advancing to the next `EntryIter` element");

        let mut device_ptr = MaybeUninit::<libblkid::blkid_dev>::zeroed();

        let result = unsafe { libblkid::blkid_dev_next(self.inner, device_ptr.as_mut_ptr()) };

        match result {
            0 => {
                log::debug!("EntryIter::next found next Device");
                let device_ptr = unsafe { device_ptr.assume_init() };
                let device = unsafe { NonNull::new_unchecked(device_ptr) };

                Some(Device::new(self.cache, device))
            }
            code => {
                log::debug!(
                        "EntryIter::next can not iterate the next element. libblkid::blkid_dev_next returned error code {:?}",
                        code
                    );

                None
            }
        }
    }
}

impl<'a> Drop for EntryIter<'a> {
    /// Disposes of the iterator.
    fn drop(&mut self) {
        log::debug!("EntryIter:: deallocating `EntryIter` instance");

        unsafe { libblkid::blkid_dev_iterate_end(self.inner) }
    }
}
