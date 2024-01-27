// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Top-level API for `LABEL` and `UUID` evaluation.

// From dependency library

// From standard library
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::path::Path;
use std::path::PathBuf;

// From this library
use crate::core::device::Tag;
use crate::core::device::TagName;
use crate::ffi_utils;

/// Returns the name of the first device with a matching `tag`. This function returns `None`,
/// if no device matching the given `tag` was found.
///
/// **Note:** Only [`Tag`]s with tag name [`TagName::Label`] and [`TagName::Uuid`] are
/// accepted; this method will return `None` if provided any other type of tag.
///
/// # Examples
///
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use std::path::PathBuf;
/// use rsblkid::core::device::Tag;
/// use rsblkid::core::utils::evaluation;
///
/// fn main() -> rsblkid::Result<()> {
///
///     let label: Tag = "LABEL='nixos'".parse()?;
///     let actual = evaluation::find_device_name_from_tag(&label);
///     let device_name = PathBuf::from("/dev/vda");
///     let expected = Some(device_name);
///
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn find_device_name_from_tag(tag: &Tag) -> Option<PathBuf> {
    // Only the `LABEL` and `UUID` tags are supported.
    if !matches!(tag.name(), TagName::Label) && !matches!(tag.name(), TagName::Uuid) {
        return None;
    }

    let key_cstr = tag.name().to_c_string();
    let value_cstr = tag.value_to_c_string().ok()?;

    log::debug!(
        "core::utils::evaluation::find_device_name_from_tag getting device name from tag: {:?}",
        tag
    );

    let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::uninit();

    unsafe {
        device_name_ptr.write(libblkid::blkid_evaluate_tag(
            key_cstr.as_ptr(),
            value_cstr.as_ptr(),
            std::ptr::null_mut(),
        ));
    }

    match unsafe { device_name_ptr.assume_init() } {
        ptr if ptr.is_null() => {
            let err_msg = format!("failed to get device name from matching tag: {:?}", tag);
            log::debug!("core::utils::evaluation::find_device_name_from_tag {}. libblkid::blkid_evaluate_tag returned a NULL pointer", err_msg);

            None
        }
        ptr => {
            let name = ffi_utils::const_c_char_array_to_path_buf(ptr);
            log::debug!(
                "core::utils::evaluation::find_device_name_from_tag found device named {:?}",
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
fn device_name_from_spec(spec: CString) -> Option<PathBuf> {
    log::debug!(
        "core::utils::evaluation::device_name_from_spec getting device name from spec {:?}",
        spec
    );

    let mut device_name_ptr = MaybeUninit::<*mut libc::c_char>::uninit();

    unsafe {
        device_name_ptr.write(libblkid::blkid_evaluate_spec(
            spec.as_ptr(),
            std::ptr::null_mut(),
        ));
    };

    match unsafe { device_name_ptr.assume_init() } {
        ptr if ptr.is_null() => {
            let err_msg = format!("failed to get device name from spec {:?}", spec);
            log::debug!("core::utils::evaluation::device_name_from_spec {}. libblkid::blkid_evaluate_spec returned a NULL pointer", err_msg);

            None
        }
        ptr => {
            let name = ffi_utils::const_c_char_array_to_path_buf(ptr);
            // Release memory allocated by `libblkid` to avoid memory leaks.
            unsafe {
                libc::free(ptr as *mut _);
            }

            log::debug!(
                "core::utils::evaluation::device_name_from_spec found device named {:?}",
                name
            );

            Some(name)
        }
    }
}

/// Returns the canonical name of the first device with a matching `tag`. A canonicalized
/// device name is an absolute path to the device where all symlinks are resolved;
/// device-mapper paths are converted to the `/dev/mapper/name` format. This function returns
/// `None`, if no device matching the given `tag` was found.
///
/// **Note:** Only [`Tag`]s with tag name [`TagName::Label`] and [`TagName::Uuid`] are
/// accepted; this method will return `None` if provided any other type of tag.
///
/// # Examples
/// ----
///
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use std::path::PathBuf;
/// use rsblkid::core::device::Tag;
/// use rsblkid::core::utils::evaluation;
///
/// fn main() -> rsblkid::Result<()> {
///     let label: Tag = r#"UUID="ac4f36bf-191b-4fb0-b808-6d7fc9fc88be""#.parse()?;
///     let actual = evaluation::find_canonical_device_name_from_tag(&label);
///     let device_name = PathBuf::from("/dev/vda");
///     let expected = Some(device_name);
///
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn find_canonical_device_name_from_tag(tag: &Tag) -> Option<PathBuf> {
    log::debug!(
        "Cache::find_canonical_device_name_from_tag getting device name matching tag {:?}",
        tag
    );

    // Only the `LABEL` and `UUID` tags are supported.
    if !matches!(tag.name(), TagName::Label) && !matches!(tag.name(), TagName::Uuid) {
        return None;
    }

    let tag_cstr = tag.to_c_string().ok()?;

    device_name_from_spec(tag_cstr)
}

/// Returns the canonical name of the first device matching the given `path`. A canonicalized
/// device name is an absolute path to the device where all symlinks are resolved;
/// device-mapper paths are converted to the `/dev/mapper/name` format. This function returns
/// `None`, if no device matching the given `path` was found.
///
/// # Examples
/// ----
///
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use std::path::PathBuf;
/// use rsblkid::core::device::Tag;
/// use rsblkid::core::utils::evaluation;
///
/// fn main() -> rsblkid::Result<()> {
///     let path = "/dev/disk/by-uuid/ac4f36bf-191b-4fb0-b808-6d7fc9fc88be";
///     let actual = evaluation::find_canonical_device_name_from_path(path);
///     let device_name = PathBuf::from("/dev/vda");
///     let expected = Some(device_name);
///
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn find_canonical_device_name_from_path<T>(path: T) -> Option<PathBuf>
where
    T: AsRef<Path>,
{
    let path = path.as_ref();
    log::debug!(
            "core::utils::evaluation::find_canonical_device_name_from_path getting device name from path {:?}",
            path
        );

    let path_cstr = ffi_utils::as_ref_path_to_c_string(path).ok()?;

    device_name_from_spec(path_cstr)
}
