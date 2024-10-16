// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;
use crate::core::errors::ConversionError;
use crate::core::partition::RawBytes;

use crate::cache::operation_enum::Operation;
use crate::cache::Builder;
use crate::cache::CacheBuilder;
use crate::cache::CacheError;
use crate::cache::Device;
use crate::cache::EntryIter;

use crate::ffi_utils;

/// Set of information about all block devices on a system.
#[derive(Debug)]
#[repr(transparent)]
pub struct Cache {
    pub(crate) inner: libblkid::blkid_cache,
}

impl<'cache> Cache {
    #[doc(hidden)]
    /// Creates a device cache.
    ///
    /// # Arguments
    ///
    /// `dest_file` -- name of the file to save changes to.
    ///
    /// If `dest_file` is set to `std::ptr::null()` changes are saved to `blkid.tab` (the default
    /// cache file).
    ///
    fn new(dest_file: *const libc::c_char) -> Result<Cache, CacheError> {
        log::debug!("Cache::new creating new `Cache` instance");

        let mut cache = MaybeUninit::<libblkid::blkid_cache>::zeroed();

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

        let mut ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

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

    /// Returns the first device with a matching `tag`. This function returns `None`,
    /// if no device with the given `tag` was found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::Path;
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
    ///     let device = cache.find_device_with_tag(&label)
    ///         .expect("found no device with tag: LABEL='nixos'");
    ///
    ///     let actual = device.name();
    ///     let expected = Path::new("/dev/vda");
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn find_device_with_tag(&mut self, tag: &Tag) -> Option<Device> {
        log::debug!(
            "Cache::find_device_with_tag trying to find device with tag: {:?}",
            tag
        );

        let key_cstr = tag.name().to_c_string();
        let value_cstr = tag.value_to_c_string().ok()?;

        let mut ptr = MaybeUninit::<libblkid::blkid_dev>::zeroed();
        unsafe {
            ptr.write(libblkid::blkid_find_dev_with_tag(
                self.inner,
                key_cstr.as_ptr(),
                value_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = format!("found no device with tag {:?}", tag);
                log::debug!("Cache::find_device_with_tag {}. blkid_find_dev_with_tag returned a NULL pointer", err_msg);

                None
            }
            device_ptr => {
                log::debug!(
                    "Cache::find_device_with_tag found device with tag: {:?}",
                    tag
                );
                let device_ptr = unsafe { NonNull::new_unchecked(device_ptr) };
                let device = Device::new(self, device_ptr);

                Some(device)
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

        let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

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

        let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::zeroed();

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

    /// Returns an iterator over the device entries in the cache.
    ///
    /// The iterator yields all items in the `Cache`, from start to end.
    ///
    /// # Panics
    ///
    /// This method panics if it is not able to instantiate a new [`EntryIter`].
    ///
    /// # Examples
    /// ----
    ///
    /// ```
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder().discard_changes_on_drop().build()?;
    ///     cache.probe_all_devices()?;
    ///
    ///     for device in cache.iter() {
    ///         println!("{}", device.name().display());
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&'cache self) -> EntryIter<'cache> {
        log::debug!("Cache::iter creating a new `EntryIter` instance");
        EntryIter::new(self).unwrap()
    }

    #[doc(hidden)]
    /// Helper function for device search by name. This is a Swiss-army knife function from `libblkid`,
    /// depending on the value of its `flag` parameter it will:
    /// - `Operation::Create`: create an empty device if `device_name` is not found in the cache,
    /// - `Operation::Find`: look-up a device entry matching `device_name` in the cache,
    /// - `Operation::Normal`: get a valid Device structure representing the named device, either
    /// from the cache or by probing `device_name`,
    /// - `Operation::Verify`: refresh data in the cache belonging to `device_name`.
    fn search_for_device_info(
        &'cache self,
        device_name: &Path,
        flag: Operation,
    ) -> Result<Device<'cache>, CacheError> {
        log::debug!(
            "Cache::search_for_device_info searching for device {:?}",
            device_name
        );

        if device_name.as_os_str().is_empty() {
            let err_msg = "can not search for device with empty name".to_owned();
            log::debug!("Cache::search_for_device_info {}", err_msg);

            return Err(CacheError::EmptyDeviceName(err_msg));
        }

        let dev_name = ffi_utils::as_ref_path_to_c_string(device_name).map_err(|e| {
            let err_msg = format!("failed to convert {:?} to a `CString`. {}", device_name, e);
            ConversionError::CString(err_msg)
        })?;

        let mut ptr = MaybeUninit::<libblkid::blkid_dev>::zeroed();

        unsafe {
            ptr.write(libblkid::blkid_get_dev(
                self.inner,
                dev_name.as_ptr(),
                flag.into(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => match flag {
                Operation::Create => {
                    let err_msg = format!("failed to create new device entry: {:?}", device_name);
                    log::debug!("Cache::search_for_device_info {}. libblkid::blkid_get_dev returned a NULL pointer", err_msg);

                    Err(CacheError::DeviceCreation(err_msg))
                }
                _otherwise => {
                    let err_msg = format!("failed to find device: {:?}", device_name);
                    log::debug!("Cache::search_for_device_info {}. libblkid::blkid_get_dev returned a NULL pointer", err_msg);

                    Err(CacheError::DeviceNotFound(err_msg))
                }
            },
            device_ptr => {
                log::debug!(
                    "Cache::search_for_device_info found device named {:?}",
                    device_name
                );

                let ptr = unsafe { NonNull::new_unchecked(device_ptr) };

                Ok(Device::new(self, ptr))
            }
        }
    }

    /// Adds a device named `device_name` to the cache, provided the device exists and one entry
    /// with the same name is not already present.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::Path;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder().discard_changes_on_drop().build()?;
    ///
    ///     let path = "/dev/vda";
    ///     let device = cache.add_new_entry(path)?;
    ///     let actual = device.name();
    ///     let expected = Path::new(path);
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn add_new_entry<T>(&'cache mut self, device_name: T) -> Result<Device<'cache>, CacheError>
    where
        T: AsRef<Path>,
    {
        log::debug!("Cache::add_new_entry adding new empty cache entry");

        Self::search_for_device_info(self, device_name.as_ref(), Operation::Create)
    }

    /// Finds a device by name, either from the cache or by probing block devices connected to the
    /// system.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use pretty_assertions::assert_eq;
    /// use std::path::Path;
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder().build()?;
    ///     cache.probe_all_devices()?;
    ///
    ///     // Search for /dev/vda
    ///     let name = "/dev/vda";
    ///     let device = cache.find_device_by_name(name)
    ///         .expect("device '/dev/vda' not found");
    ///
    ///     let actual = device.name();
    ///     let expected = Path::new(name);
    ///     assert_eq!(actual, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn find_device_by_name<T>(&'cache mut self, device_name: T) -> Option<Device<'cache>>
    where
        T: AsRef<Path>,
    {
        let device_name = device_name.as_ref();
        log::debug!(
            "Cache::find_device_by_name probing for device named {:?} ",
            device_name
        );

        Self::search_for_device_info(self, device_name, Operation::Normal).ok()
    }

    /// Finds a device by name by only searching the cache. **Does NOT refresh any cached data
    /// before searching for a device.**
    pub fn lookup_device_by_name<T>(
        &'cache mut self,
        device_name: T,
    ) -> Result<Device<'cache>, CacheError>
    where
        T: AsRef<Path>,
    {
        let device_name = device_name.as_ref();
        log::debug!(
            "Cache::lookup_device_by_name looking-up device named {:?} ",
            device_name
        );

        Self::search_for_device_info(self, device_name, Operation::Find)
    }

    /// Refreshes the cache **before** searching for a device by name.
    pub fn lookup_refreshed_device_by_name<T>(
        &'cache mut self,
        device_name: T,
    ) -> Result<Device<'cache>, CacheError>
    where
        T: AsRef<Path>,
    {
        let device_name = device_name.as_ref();
        log::debug!(
            "Cache::lookup_refreshed_device_by_name refreshing cache then looking-up device named {:?} ",
            device_name
        );

        Self::search_for_device_info(self, device_name, Operation::Verify)
    }

    /// Probes all block devices and populates the `Cache`.
    /// Checks that cached data in the `device` argument is consistent with its current state
    /// on the system, and refreshes it if necessary.
    ///
    /// For long running processes, cached data on removable devices can go stale. Use this
    /// function to refresh your copy of the `device`'s metadata if you need up-to-date
    /// information.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use rsblkid::cache::Cache;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let mut cache = Cache::builder().build()?;
    ///     let device = cache.find_device_by_name("/dev/vda")?;
    ///
    ///     // Run a long-lived process that might modify /dev/vda
    ///     // ...
    ///     // ...
    ///
    ///     // Update cached information.
    ///     let refreshed_device = cache.refresh_device_data(device);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn refresh_device_data(&'cache mut self, device: Device<'cache>) -> Device<'cache> {
        log::debug!(
            "Cache::refresh_device_data refreshing data about device named {:?} ",
            device.name()
        );

        Self::lookup_refreshed_device_by_name(self, device.name()).unwrap_or(device)
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .iter()
            .map(|device| {
                let device_tags = device
                    .iter()
                    .map(|tag| tag.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");

                format!(
                    "<device {}>{}</device>",
                    device_tags,
                    device.name().display()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
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

    const DEV_DUMMY: &'static str = "/dev/DUMMY_DEVICE";

    #[test]
    #[should_panic]
    fn add_new_entry_panics_when_device_does_not_exist() {
        let mut cache = Cache::builder().build().unwrap();

        let _ = cache.add_new_entry(DEV_DUMMY).unwrap();
    }

    #[test]
    #[should_panic]
    fn find_device_by_name_panics_when_device_does_not_exist() {
        let mut cache = Cache::builder().build().unwrap();

        let _ = cache.find_device_by_name(DEV_DUMMY).unwrap();
    }

    #[test]
    #[should_panic]
    fn lookup_device_by_name_panics_when_device_does_not_exist() {
        let mut cache = Cache::builder().build().unwrap();

        let _ = cache.lookup_device_by_name(DEV_DUMMY).unwrap();
    }

    #[test]
    #[should_panic]
    fn lookup_refreshed_device_by_name_panics_when_device_does_not_exist() {
        let mut cache = Cache::builder().build().unwrap();

        let _ = cache.lookup_refreshed_device_by_name(DEV_DUMMY).unwrap();
    }
}
