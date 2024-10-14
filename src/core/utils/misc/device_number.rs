// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;
use std::path::PathBuf;

// From this library
use crate::core::utils::misc::{Disk, MiscError};
use crate::ffi_utils;

// On Linux a file path cannot exceed 4096 characters (not accounting for the '\0' C-string terminating character).
const MAX_FILE_PATH_LENGTH: usize = 4097;

/// Returns the pathname to a block device with a given device number.
///
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use std::path::PathBuf;
/// use rsblkid::core::utils::misc;
///
/// fn main() -> rsblkid::Result<()> {
///     // Assuming we have the following devices
///     //
///     // NAME              MAJ:MIN RM   SIZE RO TYPE  MOUNTPOINTS
///     // vda                 8:16   1  14.7G  0 disk
///     // └─vda1              8:17   1     2G  0 part
///     //
///     let device_number_vda   = 0x0810; // concatenation in hexadecimal of MAJ:MIN
///     let actual = misc::device_path_from_number(device_number_vda)?;
///     let path = PathBuf::from("/dev/vda");
///     let expected = Some(path);
///     assert_eq!(actual, expected);
///
///     let device_number_vda1  = 0x0811;
///     let actual = misc::device_path_from_number(device_number_vda1)?;
///     let path = PathBuf::from("/dev/vda1");
///     let expected = "vda";
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn device_path_from_number(device_number: u64) -> Option<PathBuf> {
    log::debug!(
        "misc::device_path_from_number getting device path from device number {:?}",
        device_number
    );

    let mut ptr = MaybeUninit::<*mut libc::c_char>::zeroed();
    unsafe {
        ptr.write(libblkid::blkid_devno_to_devname(device_number));
    }

    match unsafe { ptr.assume_init() } {
        path if path.is_null() => {
            log::debug!(
                    "misc::device_path_from_number found no device path from device number {:?}. libblkid::blkid_devno_to_devname returned a NULL pointer",
                    device_number
                );

            None
        }
        path => {
            let dev_path = ffi_utils::const_c_char_array_to_path_buf(path);
            // release memory allocated by `libblkid` to avoid memory leaks.
            unsafe {
                libc::free(path as *mut _);
            }
            log::debug!(
                "misc::device_path_from_number found device path {:?} from device number {:?}",
                dev_path,
                device_number
            );

            Some(dev_path)
        }
    }
}

/// Returns a device's base name given its device number. For a partition, this function returns
/// the base name of device the partition is on.
///
/// ```ignore
/// # use pretty_assertions::assert_eq;
/// use rsblkid::core::utils::misc;
///
/// fn main() -> rsblkid::Result<()> {
///     // Assuming we have the following devices
///     //
///     // NAME              MAJ:MIN RM   SIZE RO TYPE  MOUNTPOINTS
///     // vda                 8:16   1  14.7G  0 disk
///     // └─vda1              8:17   1     2G  0 part
///     //
///     let device_number_vda   = 0x0810; // concatenation in hexadecimal of MAJ:MIN
///     let disk = misc::device_base_name_from_number(device_number_vda)?;
///     let actual = disk.name();
///     let expected = "vda";
///     assert_eq!(actual, expected);
///
///     let device_number_vda1  = 0x0811;
///     let disk = misc::device_base_name_from_number(device_number_vda1)?;
///     let actual = disk.name();
///     let expected = "vda";
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn device_base_name_from_number(device_number: u64) -> Result<Disk, MiscError> {
    log::debug!(
        "misc::device_base_name_from_number getting whole disk name from device number {:?}",
        device_number
    );

    let mut disk_name_buffer: Vec<libc::c_char> = vec![0; MAX_FILE_PATH_LENGTH];
    let mut disk_device_number = MaybeUninit::<u64>::zeroed();

    let result = unsafe {
        libblkid::blkid_devno_to_wholedisk(
            device_number,
            disk_name_buffer.as_mut_ptr(),
            MAX_FILE_PATH_LENGTH,
            disk_device_number.as_mut_ptr(),
        )
    };

    match result {
        0 => {
            let disk_name = ffi_utils::c_char_array_to_string(disk_name_buffer.as_ptr());
            let disk_device_number = unsafe { disk_device_number.assume_init() };

            log::debug!(
                "misc::device_base_name_from_number got whole disk name {:?}",
                disk_name
            );

            Ok(Disk::new(disk_name, disk_device_number))
        }
        code => {
            let err_msg = format!(
                "error getting whole disk name from device number: {}",
                device_number
            );
            log::debug!("misc::device_base_name_from_number {}. libblkid::blkid_devno_to_wholedisk returned error code {:?}", err_msg, code);

            Err(MiscError::Conversion(err_msg))
        }
    }
}
