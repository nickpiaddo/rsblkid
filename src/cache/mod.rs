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
//! accessed the first time. Device enumeration will likely miss some, that are otherwise recorded
//! in the cache. So, using the cache in this instance is highly recommended.
//!
//! All in all, there is rarely a reason not to use the cache.
//!
//! **Note:** this high-level API provides information about superblocks only (i.e. filesystems).
//! To get information about device partitions and/or topology you must use the low-level API.

pub use cache_error_enum::CacheError;

mod cache_error_enum;
