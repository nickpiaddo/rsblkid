// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Library-level error module.

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::EncodeError;
use crate::core::errors::ParserError;

use crate::cache::CacheBuilderError;
use crate::cache::CacheError;
use crate::cache::EntryIterError;
use crate::cache::TagIterError;

use crate::probe::ProbeBuilderError;
use crate::probe::ProbeError;

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
/// use rsblkid::cache::Cache;
///
/// fn main() -> rsblkid::Result<()> { // <──── automatic conversion of  ─────┐
///     //                                    error types to `BlkidError`     │
///     //                                                                    │
///     let mut cache = Cache::builder().discard_changes_on_drop().build()?;//│
///     //                                                            ^       │
///     //                                                            │       │
///     //                  might throw a `CacheBuilderError` ────────┴───────┤
///     //                                                                    │
///     cache.probe_all_devices()?;//                                         │
///     //                       ^                                            │
///     //                       │                                            │
///     //  might throw a `CacheError` ───────────────────────────────────────┘
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RsBlkidError {
    #[error(transparent)]
    Cache(#[from] CacheError),

    #[error(transparent)]
    CacheBuilder(#[from] CacheBuilderError),

    #[error(transparent)]
    Conversion(#[from] ConversionError),

    #[error(transparent)]
    Encode(#[from] EncodeError),

    #[error(transparent)]
    EntryIter(#[from] EntryIterError),

    #[error(transparent)]
    Parser(#[from] ParserError),

    #[error(transparent)]
    Probe(#[from] ProbeError),

    #[error(transparent)]
    ProbeBuilder(#[from] ProbeBuilderError),

    #[error(transparent)]
    TagIter(#[from] TagIterError),
}
