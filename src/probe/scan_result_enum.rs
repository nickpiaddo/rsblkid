// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// Result of a device scan.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ScanResult {
    /// Found device properties with conflicting values. In this case, manual intervention is advised.
    ConflictingValues,
    /// An error occurred while scanning for device properties.
    Error,
    /// Found no device properties.
    NoProperties,
    /// Found device properties.
    FoundProperties,
    /// [`Probe`](crate::probe::Probe) ended in an unexpected state while searching for device
    /// properties. Encapsulates the return code of the `libblkid` function invoked.
    Exception(i32),
}
