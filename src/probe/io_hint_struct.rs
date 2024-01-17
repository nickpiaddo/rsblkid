// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;

// From this library

/// An I/O hint.
///
/// > "Storage vendors [...] supply "I/O hints" about a device's preferred minimum unit for
/// > random I/O (`minimum_io_size`) and streaming I/O (`optimal_io_size`). For example, these
/// > hints may correspond to a RAID device's chunk size and stripe size respectively."
///
/// Source: [[Engineering Notes] I/O Limits: block sizes, alignment and I/O hints](https://access.redhat.com/articles/3911611#4)
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct IoHint {
    name: String,
    value: u64,
}

impl IoHint {
    /// Creates a new `IoHint` instance.
    pub fn new<T>(name: T, value: u64) -> IoHint
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        log::debug!(
            "IoHint::new creating a new `IoHint` instance with name: {:?} and value: {:?}",
            name,
            value
        );

        Self {
            name: name.trim().to_owned(),
            value,
        }
    }

    /// Returns the hint's name.
    pub fn name(&self) -> &str {
        log::debug!("IoHint::name hint name: {:?}", self.name);

        &self.name
    }

    /// Returns the hint's value.
    pub fn value(&self) -> u64 {
        log::debug!("IoHint::value hint value: {:?}", self.value);

        self.value
    }
}

impl<T> From<(T, u64)> for IoHint
where
    T: AsRef<str>,
{
    /// Converts a key/value tuple to a `IoHint`.
    #[inline]
    fn from(hint: (T, u64)) -> IoHint {
        let (name, value) = hint;
        IoHint::new(name, value)
    }
}

impl From<&IoHint> for IoHint {
    #[inline]
    fn from(slice: &IoHint) -> IoHint {
        slice.clone()
    }
}

impl AsRef<IoHint> for IoHint {
    #[inline]
    fn as_ref(&self) -> &IoHint {
        self
    }
}

impl fmt::Display for IoHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}
