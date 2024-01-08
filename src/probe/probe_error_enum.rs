// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::core::errors::ConversionError;

/// [`Probe`](crate::probe::Probe) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProbeError {
    /// Error while creating a new [`Probe`](crate::probe::Probe) instance.
    #[error("{0}")]
    Creation(String),

    /// Error while converting a value to a new type.
    #[error("{0}")]
    Conversion(#[from] ConversionError),

    /// Error while searching for device properties.
    #[error("{0}")]
    Search(String),

    /// Error while performing Input/Output operations.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error while configuring a [`Probe`](crate::probe::Probe).
    #[error("{0}")]
    Config(String),
}
