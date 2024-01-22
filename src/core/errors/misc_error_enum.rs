// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Encode strings to safe, udev-compatible format.

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;
use std::io;

// From this library

/// `misc` module runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MiscError {
    /// Conversion error.
    #[error("{}", .0)]
    Conversion(String),

    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("error converting to`CString`: {}", .0)]
    CStringConversion(#[from] NulError),

    /// I/O runtime error.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Error sending udev event for a block device.
    #[error("{}", .0)]
    SendUEvent(String),
}
