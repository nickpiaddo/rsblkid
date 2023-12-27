// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

use num_enum::IntoPrimitive;

#[derive(Clone, Copy, Debug, IntoPrimitive)]
#[repr(i32)]
/// Cache operation flags.
pub(crate) enum Operation {
    /// Create an empty device if not found in cache.
    Create = libblkid::BLKID_DEV_CREATE,
    /// Look-up a device entry in the cache.
    Find = libblkid::BLKID_DEV_FIND,
    /// Get a valid device structure, either from the cache or by probing the device.
    Normal = libblkid::BLKID_DEV_NORMAL,
    /// Refresh data in the cache.
    Verify = libblkid::BLKID_DEV_VERIFY,
}
