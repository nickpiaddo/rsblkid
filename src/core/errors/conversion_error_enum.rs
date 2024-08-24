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
    /// Error while converting bytes into a [`Bool`](crate::core::num::Bool).
    #[error("{0}")]
    Bool(String),

    /// Error while converting a [`CStr`](std::ffi::CStr).
    #[error("{0}")]
    CStr(String),

    /// Error while converting bytes into a [`DeviceNumber`](crate::core::device::DeviceNumber).
    #[error("{0}")]
    DeviceNumber(String),

    /// Error while converting bytes into a [`Endian`](crate::core::partition::Endian).
    #[error("{0}")]
    Endian(String),

    /// Error while converting bytes into a [`FileSystem`](crate::core::partition::FileSystem).
    #[error("{0}")]
    FileSystem(String),

    /// Error while converting bytes into a [`Guid`](crate::core::partition::Guid).
    #[error("{0}")]
    Guid(String),

    /// Error while converting bytes into a [`Id`](crate::core::device::Id).
    #[error("{0}")]
    Id(String),

    /// Error while converting bytes into a [`Label`](crate::core::device::Label).
    #[error("{0}")]
    Label(String),

    /// Error while converting bytes into a [`Name`](crate::core::device::Name).
    #[error("{0}")]
    Name(String),

    /// Error while converting bytes into a [`OSType`](crate::core::partition::OSType).
    #[error("{0}")]
    OSType(String),

    /// Error while converting bytes into a [`PartitionBitflags`](crate::core::partition::PartitionBitflags).
    #[error("{0}")]
    PartitionBitflags(String),

    /// Error while converting bytes into a [`PartitionTableType`](crate::core::partition::PartitionTableType).
    #[error("{0}")]
    PartitionTableType(String),

    /// Error while converting bytes into a [`PartitionType`](crate::core::partition::PartitionType).
    #[error("{0}")]
    PartitionType(String),

    /// Error while converting bytes into a [`Tag`](crate::core::device::Tag).
    #[error("{0}")]
    Tag(String),

    /// Error while converting bytes into a [`TagName`](crate::core::device::TagName).
    #[error("{0}")]
    TagName(String),

    /// Error while converting bytes into a [`UnixTimestamp`](crate::core::partition::UnixTimestamp).
    #[error("{0}")]
    UnixTimestamp(String),

    /// Error while converting bytes into a [`UnsignedInt`](crate::core::num::UnsignedInt).
    #[error("{0}")]
    UnsignedInt(String),

    /// Error while converting bytes into a [`Usage`](crate::core::device::Usage).
    #[error("{0}")]
    Usage(String),

    /// Error while converting bytes into a [`Uuid`](crate::core::device::Uuid).
    #[error("{0}")]
    Uuid(String),
}
