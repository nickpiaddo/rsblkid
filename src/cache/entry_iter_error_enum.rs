// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use thiserror::Error;

// From standard library

// From this library

/// [`EntryIter`](crate::cache::EntryIter) runtime errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EntryIterError {
    /// Error while creating a new [`EntryIter`](crate::cache::EntryIter).
    #[error("{0}")]
    Creation(String),
}
