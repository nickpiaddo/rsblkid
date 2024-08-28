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
    /// Error while parsing a string into an [`Endian`](crate::core::partition::Endian).
    #[error("{0}")]
    Endian(String),

    /// Error while parsing a string into a [`FileSystem`](crate::core::partition::FileSystem).
    #[error("{0}")]
    FileSystem(String),

    /// Error while parsing a string into a [`Guid`](crate::core::partition::Guid).
    #[error("{0}")]
    Guid(String),

    /// Error while parsing a string into a [`OSType`](crate::core::partition::OSType).
    #[error("{0}")]
    OSType(String),

    /// Error while parsing a string into a
    /// [`PartitionBitflags`](crate::core::partition::PartitionBitflags).
    #[error("{0}")]
    PartitionBitflags(String),

    /// Error while parsing a string into a
    /// [`PartitionTableType`](crate::core::partition::PartitionTableType).
    #[error("{0}")]
    PartitionTableType(String),

    /// Error while parsing a string into a [`PartitionType`](crate::core::partition::PartitionType).
    #[error("{0}")]
    PartitionType(String),

    /// Error while parsing a string into a [`RawBytes`](crate::core::partition::RawBytes).
    #[error("{0}")]
    RawBytes(String),

    /// Error while parsing a string into a [`UnixTimestamp`](crate::core::partition::UnixTimestamp).
    #[error("{0}")]
    UnixTimestamp(String),
}
