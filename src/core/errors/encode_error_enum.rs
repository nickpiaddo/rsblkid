// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library
use std::ffi::NulError;

// From this library

/// `encode` module runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EncodeError {
    /// Error while converting a value to [`CString`](std::ffi::CString).
    #[error("error converting to`CString`: {0}")]
    CStringConversion(#[from] NulError),

    /// Error while encoding udev-unsafe characters.
    #[error("{0}")]
    StringEncoding(String),
}
