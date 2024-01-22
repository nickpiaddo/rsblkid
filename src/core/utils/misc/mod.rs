// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Mix of various utilities for the low and high-level API.

// From dependency library

// From standard library
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::Path;

// From this library
use crate::core::errors::MiscError;
use crate::ffi_utils;

pub use device_number::*;
pub use disk_struct::Disk;
pub use library_info_struct::LibraryInfo;
pub use uevent_action_enum::UEventAction;
pub use version::*;

mod device_number;
mod disk_struct;
mod library_info_struct;
mod uevent_action_enum;
mod version;

/// Returns the size in bytes of a block device, or `0` if the [`File`] instance provides access to
/// a regular file.
pub fn device_size(block_device: &File) -> u64 {
    let size = unsafe { libblkid::blkid_get_dev_size(block_device.as_raw_fd()) as u64 };
    log::debug!("misc::device_size device size: {:?}", size);
    size
}

/// Triggers an event by adding an action to the `udev` event queue for the given block device.
///
/// # Arguments
///
/// - `device_path` -- pathname of a block device.
/// - `action` -- event to trigger.
pub fn send_uevent<T>(device_path: T, action: UEventAction) -> Result<(), MiscError>
where
    T: AsRef<Path>,
{
    log::debug!(
        "misc::send_uevent sending ACTION={:?} to {:?}",
        action,
        device_path.as_ref()
    );

    let absolute_dev_path = fs::canonicalize(&device_path)?;
    let dev_path_cstr = ffi_utils::as_ref_path_to_c_string(absolute_dev_path)?;
    let action_cstr = action.to_c_string();

    let result =
        unsafe { libblkid::blkid_send_uevent(dev_path_cstr.as_ptr(), action_cstr.as_ptr()) };

    match result {
        0 => {
            log::debug!(
                "misc::send_uevent sent ACTION={:?} to {:?}",
                action,
                device_path.as_ref()
            );

            Ok(())
        }
        code => {
            log::debug!("misc::send_uevent failed to send ACTION={:?} to {:?}. libblkid::blkid_send_uevent returned error code {:?}", action, device_path.as_ref(), code);

            Err(MiscError::SendUEvent(format!(
                "error sending udev event {:?} to {:?}",
                action,
                device_path.as_ref()
            )))
        }
    }
}
