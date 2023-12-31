// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::NonNull;

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;

use crate::cache::Cache;
use crate::cache::TagIter;

use crate::ffi_utils;

/// A block device entry in a [`Cache`].
#[derive(Debug)]
pub struct Device<'a> {
    pub(crate) inner: libblkid::blkid_dev,
    _marker: PhantomData<&'a Cache>,
}

impl<'a> Device<'a> {
    /// Creates a `Device`.
    pub(super) fn new(_: &'a Cache, device: NonNull<libblkid::blkid_struct_dev>) -> Device<'a> {
        log::debug!("Device::new creating new `Device` instance");

        Self {
            inner: device.as_ptr(),
            _marker: PhantomData,
        }
    }

    /// Returns the device's name.
    pub fn name(&self) -> &Path {
        let mut ptr = MaybeUninit::<*const libc::c_char>::uninit();
        unsafe {
            ptr.write(libblkid::blkid_dev_devname(self.inner));
        };

        // A `Device` is created if and only if its `new` method receives a NonNull pointer. We can
        // then safely assume that there is a valid device name. No need to test for a NULL
        // pointer below.
        let device_name = unsafe { ptr.assume_init() };

        let name = ffi_utils::const_c_char_array_to_path_ref(device_name);
        log::debug!("Device::name device named {:?}", name);

        name
    }

    #[doc(hidden)]
    /// Helper function:
    /// returns `true` if `device` has a tag with name `tag_name` and value `tag_value`.
    fn check_tag(
        device: libblkid::blkid_dev,
        tag_name: *const libc::c_char,
        tag_value: *const libc::c_char,
    ) -> bool {
        let result = unsafe { libblkid::blkid_dev_has_tag(device, tag_name, tag_value) };

        match result {
            1 => {
                log::debug!("Device::check_tag libblkid::blkid_dev_has_tag found tag on device");

                true
            }
            code => {
                log::debug!(
                        "Device::check_tag failed to find tag on device. libblkid::blkid_dev_has_tag returned error code: {}",
                        code
                    );
                false
            }
        }
    }

    /// Returns `true` if the `Device` has a [`Tag`] with the exact values matching the argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsblkid::core::device::Tag;
    /// use rsblkid::cache::{Cache, Device};
    /// use std::error::Error;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     let label: Tag = "LABEL=007".parse()?;
    ///     let devices: Vec<_> = cache
    ///         .iter()
    ///         .filter(|device| device.has_tag(&label))
    ///         .collect();
    ///
    ///     // We expect no device will have '007' as label.
    ///     let actual = devices.is_empty();
    ///     let expected = true;
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn has_tag<T>(&self, tag: T) -> bool
    where
        T: AsRef<Tag>,
    {
        let tag = tag.as_ref();
        let tag_name = tag.name().to_c_string();
        let tag_value = tag.value_to_c_string();

        log::debug!(
            "Device::has_tag checking if device {:?} has tag {:?}",
            self.name(),
            tag
        );

        // We assume tag name and value are valid C char arrays...
        match tag_value {
            Ok(tag_value) => Self::check_tag(self.inner, tag_name.as_ptr(), tag_value.as_ptr()),
            // ...otherwise the tag must not exist.
            Err(e) => {
                log::debug!(
                    "Device::has_tag failed to convert tag_name and/or tag_value to CString. {:?}",
                    e
                );

                false
            }
        }
    }

    /// Returns `true` if the `Device` has a [`Tag`] with a [`TagName`] matching the function argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsblkid::core::device::TagName;
    /// use rsblkid::cache::{Cache, Device};
    /// use std::error::Error;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     // Linux Unified Key Setup (LUKS) owner subsystem tag.
    ///     let tag_name = TagName::SubSystem;
    ///     let devices: Vec<_> = cache
    ///         .iter()
    ///         .filter(|device| device.has_tag_named(tag_name))
    ///         .collect();
    ///
    ///     // We expect the system not to have a LUKS device.
    ///     let actual = devices.is_empty();
    ///     let expected = true;
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn has_tag_named<T>(&self, tag_name: T) -> bool
    where
        T: AsRef<TagName>,
    {
        let tag_name = tag_name.as_ref();
        log::debug!(
            "Device::has_tag_named checking if device {:?} has tag with name {:?}",
            self.name(),
            tag_name
        );

        let c_tag_name = tag_name.to_c_string();

        Self::check_tag(self.inner, c_tag_name.as_ptr(), std::ptr::null())
    }

    /// Returns an iterator over the device tags.
    ///
    /// The iterator yields all device [`Tag`]s from start to end.
    ///
    /// # Panics
    ///
    /// This method panics if it can not instantiate an new [`TagIter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     // For each device in the cache...
    ///     for device in cache.iter() {
    ///         print!("{} :\t", device.name().display());
    ///
    ///         // display its tags.
    ///         for tag in device.iter() {
    ///             print!("{}\t", tag);
    ///         }
    ///         println!();
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&'a self) -> TagIter<'a> {
        log::debug!("Device::iter creating new `TagIter` instance");
        TagIter::new(self).unwrap()
    }
}

impl<'a> PartialEq for Device<'a> {
    /// Two `Device`s are equal when they share the same name.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.iter().eq(other.iter())
    }
}

impl<'a> Eq for Device<'a> {}

impl<'a> PartialOrd for Device<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Device<'a> {
    /// `Device`s are ordered by the lexicographical order of their names.
    ///
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by device name then by tags
        let name_cmp = self.name().cmp(other.name());
        let tags_cmp = self.iter().cmp(other.iter());

        name_cmp.then(tags_cmp)
    }
}
