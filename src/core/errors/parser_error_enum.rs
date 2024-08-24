// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// String parser runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParserError {
    /// Error while parsing a string into a [`FileSystem`](crate::core::partition::FileSystem).
    #[error("{0}")]
    FileSystem(String),

    /// Error while parsing a string into a
    /// [`PartitionTableType`](crate::core::partition::PartitionTableType).
    #[error("{0}")]
    PartitionTableType(String),
}
