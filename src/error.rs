// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Library-level error module.

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// A specialized [`Result`](std::result::Result) type for `rsblkid`.
///
/// This typedef is generally used at the program-level to avoid writing out [`RsBlkidError`]
/// directly, and is, otherwise, a direct mapping to [`Result`](std::result::Result).
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RsBlkidError>;

/// Library-level runtime errors.
///
/// This enum includes all variants of error types susceptible to occur in the library. Other, more
/// granular error types, are automatically converted to `RsBlkidError` when needed.
///
/// # Examples
/// ----
///
/// ```
/// fn main() -> rsblkid::Result<()> {
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RsBlkidError {
    #[error(transparent)]
    Conversion(#[from] ConversionError),

    #[error(transparent)]
    Parser(#[from] ParserError),
}
