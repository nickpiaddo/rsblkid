// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// Type conversion runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConversionError {
    /// Error while converting bytes into a [`PartitionTableType`](crate::core::partition::PartitionTableType).
    #[error("{0}")]
    PartitionTableType(String),
}
