// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Encode strings to safe, udev-compatible format.

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// `misc` module runtime errors.
#[derive(Clone, Debug, Error, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum MiscError {
    /// Conversion error.
    #[error("{}", .0)]
    Conversion(String),

    /// I/O runtime error.
    #[error("{}", .0)]
    IoError(String),

    /// Error sending udev event for a block device.
    #[error("{}", .0)]
    SendUEvent(String),
}
