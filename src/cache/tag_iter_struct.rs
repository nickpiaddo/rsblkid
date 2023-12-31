// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::marker::PhantomData;
use std::mem::MaybeUninit;

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;

use crate::cache::Device;
use crate::cache::TagIterError;

use crate::ffi_utils;

/// Iterator over a collection of [`Tag`]s.
#[derive(Debug)]
pub struct TagIter<'a> {
    inner: libblkid::blkid_tag_iterate,
    _marker: PhantomData<&'a Device<'a>>,
}

impl<'a> TagIter<'a> {
    /// Creates a `TagIter`.
    pub(super) fn new(device: &'a Device) -> Result<TagIter<'a>, TagIterError> {
        log::debug!("TagIter::new creating a new `TagIter` instance");

        let mut iterator = MaybeUninit::<libblkid::blkid_tag_iterate>::uninit();
        unsafe {
            iterator.write(libblkid::blkid_tag_iterate_begin(device.inner));
        };

        match unsafe { iterator.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `TagIter` instance".to_owned();
                log::debug!(
                    "TagIter::new {}. libblkid::blkid_tag_iterate_begin returned a NULL pointer",
                    err_msg
                );

                Err(TagIterError::Creation(err_msg))
            }
            inner => {
                log::debug!("TagIter::new created a new `TagIter` instance");
                let iter = Self {
                    inner,
                    _marker: PhantomData,
                };

                Ok(iter)
            }
        }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("TagIter::new advancing to next element");

        let mut tag_name_ptr = MaybeUninit::<*const libc::c_char>::uninit();
        let mut tag_value_ptr = MaybeUninit::<*const libc::c_char>::uninit();

        let result = unsafe {
            libblkid::blkid_tag_next(
                self.inner,
                tag_name_ptr.as_mut_ptr(),
                tag_value_ptr.as_mut_ptr(),
            )
        };
        match result {
            0 => {
                log::debug!("TagIter::next found next Tag");

                let tag_name_ptr = unsafe { tag_name_ptr.assume_init() };
                let tag_value_ptr = unsafe { tag_value_ptr.assume_init() };

                let cstr = ffi_utils::const_c_char_array_to_c_str(tag_name_ptr);
                let tag_name = TagName::try_from(cstr).ok()?;
                let tag_value = ffi_utils::const_c_char_array_to_bytes(tag_value_ptr);

                Tag::try_from((tag_name, tag_value)).ok()
            }
            code => {
                log::debug!("TagIter::next can not get next element. libblkid::blkid_tag_next returned error code {:?}", code);

                None
            }
        }
    }
}

impl<'a> Drop for TagIter<'a> {
    /// Disposes of the iterator.
    fn drop(&mut self) {
        log::debug!("TagIter:: deallocating `TagIter` instance");

        unsafe { libblkid::blkid_tag_iterate_end(self.inner) }
    }
}
