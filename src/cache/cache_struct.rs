// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;
use crate::core::errors::ConversionError;
use crate::core::partition::RawBytes;

use crate::cache::Builder;
use crate::cache::CacheBuilder;
use crate::cache::CacheError;

use crate::ffi_utils;

/// Set of information about all block devices on a system.
#[derive(Debug)]
#[repr(transparent)]
pub struct Cache {
    inner: libblkid::blkid_cache,
}

impl Cache {
    #[doc(hidden)]
    /// Creates a device cache.
    ///
    /// # Argument
    ///
    /// `dest_file` -- name of the file to save changes to.
    ///
    /// If `dest_file` is set to `std::ptr::null()` changes are saved to `blkid.tab` (the default
    /// cache file).
    ///
    fn new(dest_file: *const libc::c_char) -> Result<Cache, CacheError> {
        log::debug!("Cache::new creating new `Cache` instance");

        let mut cache = MaybeUninit::<libblkid::blkid_cache>::uninit();

        let result = unsafe { libblkid::blkid_get_cache(cache.as_mut_ptr(), dest_file) };

        match result {
            0 => {
                log::debug!("Cache::new created a new `Cache` instance");
                let inner = unsafe { cache.assume_init() };

                Ok(Self { inner })
            }
            code => {
                let err_msg = "failed to create a new `Cache` instance".to_owned();
                log::debug!(
                    "Cache::new {}. libblkid::blkid_get_cache: returned error code {}",
                    err_msg,
                    code
                );

                Err(CacheError::Creation(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Creates a device cache. Saves changes to the default cache file
    /// `blkid.tab` when this `Cache` instance goes out of scope.
    ///
    pub(super) fn new_default() -> Result<Cache, CacheError> {
        log::debug!(
            "Cache::new_default creating new `Cache` instance, auto saving to {:?}",
            "blkid.tab"
        );

        Self::new(std::ptr::null())
    }

    #[doc(hidden)]
    /// Creates a device cache, saving changes to `dest_file` when `Cache` goes
    /// out of scope.
    pub(super) fn new_auto_save_changes_to<P: AsRef<Path>>(
        dest_file: P,
    ) -> Result<Cache, CacheError> {
        let dest_file = dest_file.as_ref();
        log::debug!(
            "Cache::new_auto_save_changes_to creating new `Cache` instance, auto saving to {:?}",
            dest_file
        );

        let path = ffi_utils::as_ref_path_to_c_string(dest_file).map_err(|e| {
            let err_msg = format!("failed to convert {:?} to a `CString`. {}", dest_file, e);

            ConversionError::CString(err_msg)
        })?;

        Self::new(path.as_ptr())
    }

    /// Creates a [`CacheBuilder`] to configure and instantiate a `Cache`.
    ///
    /// Call the `CacheBuilder`'s [`build()`](CacheBuilder::build) method to construct a new `Cache`
    /// instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let cache_builder = Cache::builder();
    ///     // Create a `Cache` based on the default cache file `blkid.tab`,
    ///     let cache = cache_builder.build();
    ///
    ///     assert!(cache.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> CacheBuilder {
        log::debug!("Cache::builder creating new `CacheBuilder` instance");

        Builder::builder()
    }

    /// Probes all block devices, and populates the `Cache`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder().build()?;
    ///     cache.probe_all_devices()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn probe_all_devices(&mut self) -> Result<(), CacheError> {
        log::debug!("Cache::probe_all_devices probing all devices");

        let result = unsafe { libblkid::blkid_probe_all(self.inner) };

        match result {
            0 => {
                log::debug!("Cache::probe_all_devices probed all devices");

                Ok(())
            }
            code => {
                let err_msg = "failed to probe all devices".to_owned();
                log::debug!(
                        "Cache::probe_all_devices {}. libblkid::blkid_probe_all returned error code {:?}",
                        err_msg,
                        code
                    );

                Err(CacheError::ProbeError(err_msg))
            }
        }
    }

    /// Probes all new block devices, and populates the `Cache`.
    pub fn probe_all_new_devices(&mut self) -> Result<(), CacheError> {
        log::debug!("Cache::probe_all_new_devices probing all new devices");

        let result = unsafe { libblkid::blkid_probe_all_new(self.inner) };

        match result {
            0 => {
                log::debug!("Cache::probe_all_new_devices probed all new devices");

                Ok(())
            }
            code => {
                let err_msg = "failed to probe all new devices".to_owned();
                log::debug!(
                        "Cache::probe_all_new_devices {}. libblkid::blkid_probe_all_new returned error code {:?}",
                        err_msg,
                        code
                    );

                Err(CacheError::ProbeError(err_msg))
            }
        }
    }

    /// Probes all removable block devices, and populates the `Cache`.
    ///
    /// By default, device probing is based on data from `/proc/partitions`, which does not usually
    /// contain any information about removable devices (e.g. CDROMs), making them invisible to
    /// `libblkid`.
    ///
    /// `probe_all_removable_devices` adds the metadata about all removable block devices to the
    /// cache, with probing based on data located in the `/sys` directory.
    ///
    /// - **Warning:** probing removable devices (floppies, CDROMs, ...) is an operation that can
    /// take a long time to complete. Therefore, it is **strongly** advised **against** calling
    /// this function repeatedly.
    ///
    /// - **Note:** devices detected by this function, will not be saved to the default `blkid.tab`
    /// cache file when a `Cache` instance goes out of scope.
    ///
    pub fn probe_all_removable_devices(&mut self) -> Result<(), CacheError> {
        log::debug!("Cache::probe_all_removable_devices probing all removable devices");

        let result = unsafe { libblkid::blkid_probe_all_removable(self.inner) };

        match result {
            0 => {
                log::debug!("Cache::probe_all_removable_devices probed all removable devices");

                Ok(())
            }
            code => {
                let err_msg = "failed to probe all removable devices".to_owned();
                log::debug!("Cache::probe_all_removable_devices {}. libblkid::blkid_probe_all_removable returned error code {}", err_msg, code);

                Err(CacheError::ProbeError(err_msg))
            }
        }
    }

    /// Removes stale data about devices that are no-longer connected to the system.
    pub fn garbage_collect(&mut self) {
        log::debug!("Cache::garbage_collect removing stale data from cache");
        unsafe { libblkid::blkid_gc_cache(self.inner) }
    }

    /// Returns the value of the tag named `tag_name` on a specific device at `path`, `None` if the
    /// device does not have a tag matching the given name.
    ///
    /// **Note:** Only [`Tag`]s with tag name [`Tag::Label`] and [`Tag::Uuid`] are
    /// accepted; this method will return `None` if provided any other type of tag.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use rsblkid::core::device::TagName;
    /// use rsblkid::core::partition::RawBytes;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     let tag_name = TagName::Label;
    ///     let path = "/dev/vda";
    ///     let actual = cache.tag_value_from_device(&tag_name, path);
    ///     let value = RawBytes::from(b"nixos".to_vec());
    ///     let expected = Some(value);
    ///
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn tag_value_from_device<T>(&self, tag_name: &TagName, path: T) -> Option<RawBytes>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref();
        log::debug!("Cache::tag_value_from_device trying to find the value of tag named: {:?} for device: {:?}", tag_name, path);
        // Only the `LABEL` and `UUID` tags are supported.
        if !matches!(tag_name, TagName::Label) && !matches!(tag_name, TagName::Uuid) {
            return None;
        }

        let key_cstr = tag_name.to_c_string();
        let path_cstr = ffi_utils::as_ref_path_to_c_string(path).ok()?;

        let mut ptr = MaybeUninit::<*mut libc::c_char>::uninit();

        unsafe {
            ptr.write(libblkid::blkid_get_tag_value(
                self.inner,
                key_cstr.as_ptr(),
                path_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!(
                    "device at {:?} does not have a tag named: {:?}",
                    path.display(),
                    tag_name
                );
                log::debug!(
                    "Cache::tag_value_from_device {}. blkid_get_tag_value returned a NULL pointer",
                    err_msg
                );

                None
            }
            value_ptr => {
                let value = ffi_utils::const_c_char_array_to_bytes(value_ptr);
                let value = RawBytes::from(value);
                log::debug!(
                    "Cache::tag_value_from_device for device {:?}: {}='{}'",
                    path,
                    tag_name,
                    value
                );

                Some(value)
            }
        }
    }

    /// Returns the name of the first device with a matching `tag`. This function returns `None`,
    /// if no device matching the given `tag` was found.
    ///
    /// **Note:** Only [`Tag`]s with tag name [`Tag::Label`] and [`Tag::Uuid`] are
    /// accepted; this method will return `None` if provided any other type of tag.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::PathBuf;
    /// use rsblkid::core::device::Tag;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     let label: Tag = "LABEL='nixos'".parse()?;
    ///     let actual = cache.find_device_name_from_tag(&label);
    ///     let device_name = PathBuf::from("/dev/vda");
    ///     let expected = Some(device_name);
    ///
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn find_device_name_from_tag(&mut self, tag: &Tag) -> Option<PathBuf> {
        // Only the `LABEL` and `UUID` tags are supported.
        if !matches!(tag.name(), TagName::Label) && !matches!(tag.name(), TagName::Uuid) {
            return None;
        }

        let key_cstr = tag.name().to_c_string();
        let value_cstr = tag.value_to_c_string().ok()?;

        log::debug!(
            "Cache::find_device_name_from_tag getting device name from tag: {:?}",
            tag
        );

        let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::uninit();

        unsafe {
            device_name_ptr.write(libblkid::blkid_evaluate_tag(
                key_cstr.as_ptr(),
                value_cstr.as_ptr(),
                &mut self.inner,
            ));
        }

        match unsafe { device_name_ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("failed to get device name from matching tag: {:?}", tag);
                log::debug!("Cache::find_device_name_from_tag {}. libblkid::blkid_evaluate_tag returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                let name = ffi_utils::const_c_char_array_to_path_buf(ptr);
                log::debug!(
                    "Cache::find_device_name_from_tag found device named {:?}",
                    name
                );

                // Release memory allocated by `libblkid` to avoid memory leaks.
                unsafe {
                    libc::free(ptr as *mut _);
                }

                Some(name)
            }
        }
    }

    #[doc(hidden)]
    /// Returns the canonical name of the first device matching the given `spec`, which is
    /// either a [`Tag`] or a [`Path`] as a [`CString`]. A canonicalized device name is an absolute
    /// path to the device where all symlinks are resolved; device-mapper paths are converted to
    /// the `/dev/mapper/name` format.
    fn device_name_from_spec(cache: &mut Self, spec: CString) -> Option<PathBuf> {
        log::debug!(
            "Cache::device_name_from_spec getting device name from spec {:?}",
            spec
        );

        let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::uninit();

        unsafe {
            device_name_ptr.write(libblkid::blkid_evaluate_spec(
                spec.as_ptr(),
                &mut cache.inner,
            ));
        };

        match unsafe { device_name_ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("failed to get device name from spec {:?}", spec);
                log::debug!("Cache::device_name_from_spec {}. libblkid::blkid_evaluate_spec returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                let name = ffi_utils::const_c_char_array_to_path_buf(ptr);
                // Release memory allocated by `libblkid` to avoid memory leaks.
                unsafe {
                    libc::free(ptr as *mut _);
                }

                log::debug!("Cache::device_name_from_spec found device named {:?}", name);

                Some(name)
            }
        }
    }

    /// Returns the canonical name of the first device with a matching `tag`. A canonicalized
    /// device name is an absolute path to the device where all symlinks are resolved;
    /// device-mapper paths are converted to the `/dev/mapper/name` format. This function returns
    /// `None`, if no device matching the given `tag` was found.
    ///
    /// **Note:** Only [`Tag`]s with tag name [`Tag::Label`] and [`Tag::Uuid`] are
    /// accepted; this method will return `None` if provided any other type of tag.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::PathBuf;
    /// use rsblkid::core::device::Tag;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     let uuid: Tag = r#"UUID="ac4f36bf-191b-4fb0-b808-6d7fc9fc88be""#.parse()?;
    ///     let actual = cache.find_canonical_device_name_from_tag(&uuid);
    ///     let device_name = PathBuf::from("/dev/vda");
    ///     let expected = Some(device_name);
    ///
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn find_canonical_device_name_from_tag(&mut self, tag: &Tag) -> Option<PathBuf> {
        log::debug!(
            "Cache::find_canonical_device_name_from_tag getting device name matching tag {:?}",
            tag
        );

        // Only the `LABEL` and `UUID` tags are supported.
        if !matches!(tag.name(), TagName::Label) && !matches!(tag.name(), TagName::Uuid) {
            return None;
        }

        let tag_cstr = tag.to_c_string().ok()?;

        Self::device_name_from_spec(self, tag_cstr)
    }

    /// Returns the canonical name of the first device matching the given `path`. A canonicalized
    /// device name is an absolute path to the device where all symlinks are resolved;
    /// device-mapper paths are converted to the `/dev/mapper/name` format. This function returns
    /// `None`, if no device matching the given `path` was found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::PathBuf;
    /// use rsblkid::core::device::Tag;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder()
    ///         .discard_changes_on_drop()
    ///         .build()?;
    ///
    ///     cache.probe_all_devices()?;
    ///
    ///     let symlink = "/dev/disk/by-uuid/ac4f36bf-191b-4fb0-b808-6d7fc9fc88be";
    ///     let actual = cache.find_canonical_device_name_from_path(symlink);
    ///     let device_name = PathBuf::from("/dev/vda");
    ///     let expected = Some(device_name);
    ///
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn find_canonical_device_name_from_path<T>(&mut self, path: T) -> Option<PathBuf>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref();
        log::debug!(
            "Cache::find_canonical_device_name_from_path getting device name from path {:?}",
            path
        );

        let path_cstr = ffi_utils::as_ref_path_to_c_string(path).ok()?;

        Self::device_name_from_spec(self, path_cstr)
    }
}

impl Drop for Cache {
    /// Saves changes to device information into the destination file provided at construction.
    fn drop(&mut self) {
        log::debug!("Cache::drop deallocate `Cache` instance`");

        unsafe { libblkid::blkid_put_cache(self.inner) }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(
        expected = "can not set `discard_changes_on_drop` and `auto_save_changes_to` simultaneously"
    )]
    fn discard_changes_on_drop_and_auto_save_changes_are_mutually_exclusive() {
        let _ = Cache::builder()
            .discard_changes_on_drop()
            .auto_save_changes_to("/dev/null")
            .build()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "can not set `discard_changes_on_drop` and `auto_save_changes_to` simultaneously"
    )]
    fn reordered_discard_changes_on_drop_and_auto_save_changes_are_mutually_exclusive() {
        let _ = Cache::builder()
            .auto_save_changes_to("/dev/null")
            .discard_changes_on_drop()
            .build()
            .unwrap();
    }
}
