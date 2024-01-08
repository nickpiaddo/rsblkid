// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library
use crate::probe::ProbeError;

/// [`ProbeBuilder`](crate::probe::ProbeBuilder) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProbeBuilderError {
    /// Error while creating a new [`Probe`](crate::probe::Probe) instance.
    #[error(transparent)]
    ProbeBuild(#[from] ProbeError),

    /// Error if two mutually exclusive setter functions are called.
    #[error("{}", .0)]
    MutuallyExclusive(String),

    /// Error if failed to call a mandatory setter function.
    #[error("{}", .0)]
    Required(String),
}
