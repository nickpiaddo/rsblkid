// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library
use std::fmt;

// From this library

/// Block device scanning filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive)]
#[non_exhaustive]
#[repr(i32)]
pub enum Filter {
    /// Scan for all items matching the filter.
    In = libblkid::BLKID_FLTR_ONLYIN,
    /// Scan for all items **NOT** matching the filter.
    Out = libblkid::BLKID_FLTR_NOTIN,
}

impl Filter {
    /// View this `Filter` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Filter::In => "In",
            Filter::Out => "Out",
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
