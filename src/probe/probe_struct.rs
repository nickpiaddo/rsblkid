// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fs::{File, OpenOptions};
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

// From this library
use crate::core::errors::ConversionError;
use crate::probe::IoHint;
use crate::probe::PrbBuilder;
use crate::probe::ProbeBuilder;
use crate::probe::ProbeError;

use crate::ffi_utils;

/// Low-level device probe.
#[derive(Debug)]
pub struct Probe {
    pub(crate) inner: libblkid::blkid_probe,
    file: File,
    #[allow(dead_code)]
    is_read_only: bool,
}

impl Probe {
    /// Creates a [`ProbeBuilder`] to configure and construct a new`Probe` instance.
    ///
    /// Call the `ProbeBuilder`'s [`build()`](ProbeBuilder::build) method to construct a new `Probe`
    /// instance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use rsblkid::probe::Probe;
    ///
    /// fn main() -> rsblkid::Result<()> {
    ///     let probe_builder = Probe::builder();
    ///
    ///     let probe = probe_builder
    ///         .scan_device("/dev/vda")
    ///         .build();
    ///
    ///     assert!(probe.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> ProbeBuilder {
        log::debug!("Probe::builder creating new `ProbeBuilder` instance");

        PrbBuilder::builder()
    }

    #[doc(hidden)]
    /// Associate a device to a new blkid_probe C struct.
    /// FIXME
    /// libblkid::blkid_probe_set_device does not deallocate the file by default if the
    /// BLKID_FL_PRIVATE_FD flag is not set in the blkid_probe struct
    /// see util-linux/libblkid/src/probe.c:889
    /// unless FDGETFDCSTAT is defined
    /// see util-linux/libblkid/src/probe.c:977 -> 1006
    /// POTENTIAL DOUBLE FREE RISK??
    /// Assign a device file descriptor to the probe, reset its internal buffers,
    /// state, and close the previously associated device.
    ///
    /// # Arguments
    ///
    /// - ptr -- pointer to a libblkid probe structure.
    /// - file -- `File` object associated to the device to probe.
    /// - location -- location of the region to probe (offset from the start of the device).
    /// - size -- size of the region to probe (a value of `0` <=> probe the whole device/file).
    ///
    fn set_device(
        ptr: libblkid::blkid_probe,
        file: &mut File,
        location: u64,
        size: u64,
    ) -> Result<(), ProbeError> {
        log::debug!("Probe::set_device setting device to scan");

        let result = unsafe {
            libblkid::blkid_probe_set_device(ptr, file.as_raw_fd(), location as i64, size as i64)
        };

        match result {
            0 | 1 => {
                log::debug!("Probe::set_device set device to scan");

                Ok(())
            }
            code => {
                let err_msg = "failed to set device to scan".to_owned();
                log::debug!(
                        "Probe::set_device {}. libblkid::blkid_probe_set_device returned error code {:?}",
                        err_msg,
                        code
                    );

                Err(ProbeError::Config(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Returns a new read only `Probe` on a device.
    pub(crate) fn new_read_only<T>(
        file_name: T,
        scan_segment: (u64, u64),
    ) -> Result<Probe, ProbeError>
    where
        T: AsRef<Path>,
    {
        let file_name = file_name.as_ref();
        let (location, size) = &scan_segment;
        log::debug!(
            "Probe::new_read_only creating a new Probe in read only mode associated with {:?} scanning the {}",
            file_name,
            if scan_segment == (0, 0) {
                "whole device".to_owned()
            } else {
                format!("region (location: {}, size: {} bytes)", location, size)
            }
        );

        // Custom flags taken from util-linux/libblkid/src/probe.c:215
        let status_flags = libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NONBLOCK;
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(status_flags)
            .open(file_name)?;

        Self::new(file, scan_segment, false)
    }

    #[doc(hidden)]
    /// Returns a new read-write `Probe` on a device.
    pub(crate) fn new_read_write<T>(
        file_name: T,
        scan_segment: (u64, u64),
    ) -> Result<Probe, ProbeError>
    where
        T: AsRef<Path>,
    {
        let file_name = file_name.as_ref();
        let (location, size) = &scan_segment;
        log::debug!(
            "Probe::new_read_write creating a new Probe in read/write mode associated with {:?} scanning the {}",
            file_name,
            if scan_segment == (0, 0) {
                "whole device".to_owned()
            } else {
                format!("region (location: {}, size: {} bytes)", location, size)
            }
        );

        // Custom flags taken from util-linux/libblkid/src/probe.c:215
        let status_flags = libc::O_RDWR | libc::O_CLOEXEC;
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(status_flags)
            .open(file_name)?;

        Self::new(file, scan_segment, false)
    }

    #[doc(hidden)]
    /// Returns a new `Probe` instance from a `File` object.
    pub(crate) fn new_from_file(file: File, scan_segment: (u64, u64)) -> Result<Probe, ProbeError> {
        log::debug!("Probe::new_from_file creating new `Probe` instance from `File`");

        Self::new(file, scan_segment, true)
    }

    #[doc(hidden)]
    /// Returns a new `Probe` instance from a `File` object.
    pub(crate) fn new_from_file_read_write(
        file: File,
        scan_segment: (u64, u64),
    ) -> Result<Probe, ProbeError> {
        log::debug!("Probe::new_from_file_read_write creating new `Probe` instance from `File`");

        if ffi_utils::is_open_read_write(&file)? {
            Self::new(file, scan_segment, false)
        } else {
            let err_msg =
                "failed to create a `Probe` in read/write mode from a read-only `File`".to_owned();

            Err(ProbeError::Creation(err_msg))
        }
    }

    #[doc(hidden)]
    /// Returns a new `Probe` linked to a device.
    pub(crate) fn new(
        mut file: File,
        scan_segment: (u64, u64),
        is_read_only: bool,
    ) -> Result<Probe, ProbeError> {
        let (location, size) = scan_segment;
        let mut probe = MaybeUninit::<libblkid::blkid_probe>::uninit();

        // Allocate a new blkid_probe C struct.
        unsafe {
            probe.write(libblkid::blkid_new_probe());
        }

        match unsafe { probe.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to create a new `Probe` instance".to_owned();
                log::debug!(
                    "Probe::new {}. libblkid::blkid_new_probe returned a NULL pointer",
                    err_msg
                );

                Err(ProbeError::Creation(err_msg))
            }
            inner => {
                Self::set_device(inner, &mut file, location, size)?;

                log::debug!("Probe::new created a new `Probe` instance");

                Ok(Self {
                    inner,
                    file,
                    is_read_only,
                })
            }
        }
    }

    /// Returns the associated block device's identification number (`0` for a regular file).
    pub fn device_number(&self) -> u64 {
        let dev_num = unsafe { libblkid::blkid_probe_get_devno(self.inner) };
        log::debug!("Probe::device_number device has ID number: {:?}", dev_num);

        dev_num
    }

    /// Returns a reference to the [`File`] object associated with the device being scanned.
    pub fn device_file(&self) -> &File {
        log::debug!("Probe::device_file return `File` object reference");

        &self.file
    }

    /// Returns the size of the associated block device in 512-byte sectors.
    pub fn device_size_in_sectors(&self) -> u64 {
        log::debug!("Probe::device_size_in_sectors getting block device size (sectors)");

        let size = unsafe { libblkid::blkid_probe_get_sectors(self.inner) };

        log::debug!(
            "Probe::device_size_in_sectors device size (sectors): {:?}",
            size
        );

        size as u64
    }

    /// Returns the size in bytes of the associated block device.
    pub fn device_size(&self) -> u64 {
        log::debug!("Probe::device_size getting block device size (bytes)");

        let size = self.device_size_in_sectors() * 512;

        log::debug!("Probe::device_size block device size (bytes): {:?}", size);

        size
    }

    /// Returns the size in bytes of a logical sector on the associated block device.
    pub fn device_logical_sector_size(&self) -> usize {
        let size = unsafe { libblkid::blkid_probe_get_sectorsize(self.inner) };
        log::debug!(
            "Probe::device_logical_sector_size logical sector size (bytes): {:?}",
            size
        );

        size as usize
    }

    /// Returns the identification number assigned to the whole disk containing the associated block device.
    ///
    /// Returns `0` for a regular file.
    pub fn device_whole_disk_number(&self) -> u64 {
        let disk_num = unsafe { libblkid::blkid_probe_get_wholedisk_devno(self.inner) };
        log::debug!(
            "Probe::device_whole_disk_number disk identification number: {:?}",
            disk_num
        );

        disk_num
    }

    /// Defines a segment of bytes to skip while scanning the associated block device. Data in
    /// memory buffers matching the given range are filled with zeros.
    ///
    /// - **Warning:** configuration about segments to skip is discarded when the function
    /// [`Probe::empty_buffers`] is called.
    ///
    /// # Arguments
    ///
    /// - `from` -- location (in bytes) of the segment to skip (i.e. offset).
    /// - `length` -- length of the segment to skip.
    pub fn device_skip_bytes(&mut self, from: u64, length: u64) -> Result<(), ProbeError> {
        log::debug!(
            "Probe::device_skip_bytes skipping segment (from: {:?}, length: {:?})",
            from,
            length
        );

        let result = unsafe { libblkid::blkid_probe_hide_range(self.inner, from, length) };

        match result {
            0 => {
                log::debug!(
                    "Probe::device_skip_bytes skipped segment (from: {:?}, length: {:?})",
                    from,
                    length
                );

                Ok(())
            }
            code => {
                let err_msg = format!(
                    "failed to skip segment (from: {}, length: {})",
                    from, length
                );
                log::debug!("Probe::device_skip_bytes {}. libblkid::blkid_probe_hide_range returned error code {:?}", err_msg, code);

                Err(ProbeError::Config(err_msg))
            }
        }
    }

    /// Returns `true` when the device associated to the `Probe` is a whole disk instead of a partition.
    pub fn is_device_whole_disk(&self) -> bool {
        let res = unsafe { libblkid::blkid_probe_is_wholedisk(self.inner) == 1 };
        log::debug!("Probe::is_device_whole_disk {}", res);

        res
    }

    /// Returns the location of the segment being scanned with respect to the device's first byte.
    pub fn scanned_device_segment_location(&self) -> u64 {
        log::debug!(
            "Probe::scanned_device_segment_location getting scanned segment location (bytes)"
        );

        let location = unsafe { libblkid::blkid_probe_get_offset(self.inner) };
        log::debug!(
            "Probe::scanned_device_segment_location scanned segment location (bytes): {:?}",
            location
        );

        location as u64
    }

    /// Returns the size in bytes of the segment being scanned.
    ///
    /// Returns the size of the whole block device when no limits were defined for the region to scan.
    pub fn scanned_device_segment_size(&self) -> u64 {
        log::debug!("Probe::scanned_device_segment_size getting scanned segment size (bytes)");

        let size = unsafe { libblkid::blkid_probe_get_size(self.inner) };
        log::debug!(
            "Probe::scanned_device_segment_size scanned segment size (bytes): {}",
            size
        );

        size as u64
    }

    /// Resets and frees all cached buffers.
    ///
    /// For performance, `Probe` maintains an in-memory cache of the characteristics of its
    /// associated device. Calling this function will invalidate the cache's buffers.
    pub fn empty_buffers(&mut self) -> Result<(), ProbeError> {
        log::debug!("Probe::empty_buffers emptying buffers");

        let result = unsafe { libblkid::blkid_probe_reset_buffers(self.inner) };

        match result {
            0 => {
                log::debug!("Probe::empty_buffers emptied buffers");
                Ok(())
            }

            code => {
                let err_msg = "failed to empty buffers".to_owned();
                log::debug!(
                        "Probe::empty_buffers {}. libblkid::blkid_probe_reset_buffers returned error code {:?}",
                        err_msg,
                        code
                    );

                Err(ProbeError::Config(err_msg))
            }
        }
    }

    /// Sets I/O hints about a device.
    ///
    /// Some legacy devices do not provide I/O hints, this function allows you to define the
    /// missing values for optimal performance.
    ///
    /// Currently, the only I/O hint supported by the library is `"session_offset"` for designating
    /// the location (in bytes) of a session on a multi-session device in  Universal Disk Format (UDF).
    pub fn set_hint(&mut self, hint: &IoHint) -> Result<(), ProbeError> {
        let hint_cstr = hint.name_to_c_string().map_err(|e| {
            let err_msg = format!("failed to convert {:?} to a `CString`. {}", hint.name(), e);
            ConversionError::CString(err_msg)
        })?;
        let value = hint.value();

        log::debug!(
            "Probe::set_hint setting hint {:?} with value {:?}",
            hint,
            value
        );

        let result =
            unsafe { libblkid::blkid_probe_set_hint(self.inner, hint_cstr.as_ptr(), value) };

        match result {
            0 => {
                log::debug!("Probe::set_hint set hint {:?} with value {:?}", hint, value);

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set hint {:?} with velue {:?}", hint, value);
                log::debug!(
                    "Probe::set_hint {}. libblkid::blkid_probe_set_hint returned error code {:?}",
                    err_msg,
                    code
                );

                Err(ProbeError::Config(err_msg))
            }
        }
    }

    /// Discards all hints set by [`Probe::set_hint`].
    pub fn discard_hints(&mut self) {
        log::debug!("Probe::discard_hints discarding hints");

        unsafe { libblkid::blkid_probe_reset_hints(self.inner) }
    }

    #[doc(hidden)]
    /// Sets how many consecutive bytes amount to a sector.
    /// Note that blkid_probe_set_device() resets this setting. Use it after
    /// blkid_probe_set_device() and before any probing call.
    pub(crate) fn set_bytes_per_sector(&self, size: u32) -> Result<(), ProbeError> {
        log::debug!("Probe::set_bytes_per_sector setting sector size");

        let result = unsafe { libblkid::blkid_probe_set_sectorsize(self.inner, size) };

        match result {
            0 => {
                log::debug!(
                    "Probe::set_bytes_per_sector set bytes per sector to {:?}",
                    size
                );

                Ok(())
            }
            code => {
                let err_msg = format!("failed to set bytes per sector to: {:?}", size);
                log::debug!("Probe::set_bytes_per_sector {}. libblkid::blkid_probe_set_sectorsize returned error code {}", err_msg, code);

                Err(ProbeError::Config(err_msg))
            }
        }
    }

    /// Sets the current position in the sequence of search functions to that of the one executed
    /// before last.
    pub fn backtrack(&mut self) -> Result<(), ProbeError> {
        log::debug!("Probe::backtrack backtracking to previous search function");

        let result = unsafe { libblkid::blkid_probe_step_back(self.inner) };

        match result {
            0 => {
                log::debug!("Probe::backtrack backtracked to previous search function");

                Ok(())
            }
            code => {
                let err_msg = "failed to backtrack to previous search function".to_owned();
                log::debug!(
                    "Probe::backtrack {}. libblkid::blkid_probe_step_back returned error code: {}",
                    err_msg,
                    code
                );

                Err(ProbeError::Search(err_msg))
            }
        }
    }

    /// Reverts the `Probe` to its state at creation.
    pub fn reset(&mut self) {
        log::debug!("Probe::reset resetting `Probe`");

        unsafe { libblkid::blkid_reset_probe(self.inner) }
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        log::debug!("Probe:: deallocating probe instance");

        unsafe { libblkid::blkid_free_probe(self.inner) }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "one of the options `scan_device` or `scan_file` must be set")]
    fn probe_one_of_scan_device_or_scan_file_must_be_set() {
        let _ = Probe::builder().build().unwrap();
    }

    #[test]
    #[should_panic(expected = "can not set `scan_device` and `scan_file` simultaneously")]
    fn probe_scan_device_and_scan_file_are_mutually_exclusive() {
        let tmp_file = tempfile::tempfile().unwrap();
        let _ = Probe::builder()
            .scan_device("/dev/vda")
            .scan_file(tmp_file)
            .build()
            .unwrap();
    }
}
