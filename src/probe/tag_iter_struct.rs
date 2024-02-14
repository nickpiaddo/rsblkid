// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;
use crate::ffi_utils;
use crate::probe::Probe;

/// Iterator over a collection of [`Tag`]s.
#[derive(Debug)]
pub struct TagIter<'a> {
    probe: &'a Probe,
    index: i32,
}

impl<'a> TagIter<'a> {
    /// Create a new `TagIter` instance.
    #[allow(dead_code)]
    pub(super) fn new(probe: &'a Probe) -> TagIter<'a> {
        log::debug!("TagIter::new creating a new `TagIter` instance");
        Self { probe, index: 0 }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("TagIter::next iterating next element");

        let mut tag_name_ptr = MaybeUninit::<*const libc::c_char>::uninit();
        let mut tag_value_ptr = MaybeUninit::<*const libc::c_char>::uninit();

        let result = unsafe {
            libblkid::blkid_probe_get_value(
                self.probe.inner,
                self.index,
                tag_name_ptr.as_mut_ptr(),
                tag_value_ptr.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };

        match result {
            0 => {
                log::debug!("TagIter::next found next element");
                self.index += 1;
                let tag_name_ptr = unsafe { tag_name_ptr.assume_init() };
                let tag_value_ptr = unsafe { tag_value_ptr.assume_init() };

                let cstr = ffi_utils::const_c_char_array_to_c_str(tag_name_ptr);
                let tag_name = TagName::try_from(cstr).ok()?;
                let tag_value = ffi_utils::const_c_char_array_to_bytes(tag_value_ptr);

                Tag::try_from((tag_name, tag_value)).ok()
            }
            code => {
                log::debug!("TagIter::next can not get next element. libblkid::blkid_probe_get_value returned error code {:?}", code);

                None
            }
        }
    }
}
