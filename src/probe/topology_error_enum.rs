// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`Topology`](crate::probe::Topology) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TopologyError {
    /// Error while creating a new [`Topology`](crate::probe::Topology) instance.
    #[error("{}", .0)]
    Creation(String),
}
