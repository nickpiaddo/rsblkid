// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::ConversionError;

/// [`Cache`](crate::cache::Cache) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CacheError {
    /// Error during a [`Cache`](crate::cache::Cache) initialization.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to a new type.
    #[error("{0}")]
    Conversion(#[from] ConversionError),

    /// Error while probing block devices.
    #[error("{0}")]
    ProbeError(String),
}
