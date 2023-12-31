// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! High-level API to handle device identification and tag extraction.
//!
//! ----
//!
//! `libblkid` usually keeps all block device information in a cache file: `blkid.tab`. Provided a
//! user has read permission on a raw block device, `libblkid` verifies that data is still fresh
//! before returning it to the user.
//!
//! The cache file allows unprivileged users, as well as those **not** members of the `disk` group,
//! to locate devices by label/id.
//!
//! We strongly recommended you use the cache if you are dealing with multiple devices (even
//! empty ones), as devices will be scanned at most once, and the on-disk cache will be updated if
//! possible.
//!
//! In situations where one is seeking information about a single known device, using the cache (or
//! not) doesn't have an impact on performance; unless you are not allowed to read the block device
//! directly.
//!
//! In some cases (e.g. modular kernels), block devices are not even visible until after they are
//!
//! All in all, there is rarely a reason not to use the cache.
//!
//! **Note:** this high-level API provides information about superblocks only (i.e. filesystems).
//! To get information about device partitions and/or topology you must use the low-level API.
//!
//! # Examples
//!
//! ```
//! use tempfile::NamedTempFile;
//! use rsblkid::cache::Cache;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Named temporary file.
//!     let temp_file = NamedTempFile::new()?;
//!
//!     // Configure, then create a cache object that saves changes to a custom cache file when it goes
//!     // out of scope.
//!     let mut custom_cache = Cache::builder()
//!         .auto_save_changes_to(temp_file.path())
//!         .build()?;
//!
//!     // Probe all block devices, and populate the Cache.
//!     custom_cache.probe_all_devices()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Example of a cache file's content:
//!
//! ```xml
//! <device DEVNO="0xfe01" TIME="1687337407.788618" PRI="45" LABEL="root" UUID="9e4adae9-4122-47fe-848f-67a9eb726207" BLOCK_SIZE="4096" TYPE="ext4">/dev/mapper/vg_nixos-root</device>
//! <device DEVNO="0x0811" TIME="1687337407.869044" LABEL_FATBOOT="ESP" LABEL="ESP" UUID="9DE0-4F47" BLOCK_SIZE="512" TYPE="vfat" PARTLABEL="ESP" PARTUUID="09438be4-b083-4efc-ad7d-2b5f7abe929f">/dev/sdb1</device>
//! <device DEVNO="0xfe02" TIME="1687337407.929706" PRI="45" LABEL="swap" UUID="8ff79303-112d-4412-8311-8400f133d294" TYPE="swap">/dev/mapper/vg_nixos-swap</device>
//! <device DEVNO="0xfe00" TIME="1687337407.987098" PRI="40" UUID="fpJpfp-vsta-XKCT-8Esn-Ih6V-ifjx-IOm5TL" TYPE="LVM2_member">/dev/mapper/lukscontainer</device>
//! <device DEVNO="0x0840" TIME="1687337408.40832" UUID="38059df8-291d-4d92-827e-255a1020f3e0" BLOCK_SIZE="4096" TYPE="ext4">/dev/sde</device>
//! <device DEVNO="0xfe03" TIME="1687337408.80023" PRI="45" LABEL="home" UUID="144784a5-82a5-41ce-a018-3b41da764bc7" BLOCK_SIZE="4096" TYPE="ext4">/dev/mapper/vg_nixos-home</device>
//! ```

pub use cache_builder_error_enum::CacheBuilderError;
pub(crate) use cache_builder_struct::Builder;
pub use cache_builder_struct::CacheBuilder;
pub use cache_error_enum::CacheError;
pub use cache_struct::Cache;
pub use device_struct::Device;
pub use entry_iter_error_enum::EntryIterError;
pub use entry_iter_struct::EntryIter;
pub use tag_iter_error_enum::TagIterError;
pub use tag_iter_struct::TagIter;

mod cache_builder_error_enum;
mod cache_builder_struct;
mod cache_error_enum;
mod cache_struct;
mod device_struct;
mod entry_iter_error_enum;
mod entry_iter_struct;
mod operation_enum;
mod tag_iter_error_enum;
mod tag_iter_struct;
