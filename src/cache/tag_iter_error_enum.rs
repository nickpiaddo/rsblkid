// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`TagIter`](crate::cache::TagIter) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TagIterError {
    /// Error while creating a new [`TagIter`](crate::cache::TagIter).
    #[error("{0}")]
    Creation(String),
}
