// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

use crate::core::num::UnsignedInt;

/// Distance from the beginning of a device.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Offset(UnsignedInt);

impl Offset {
    /// Returns the `Offset` value.
    pub fn value(&self) -> &UnsignedInt {
        &self.0
    }

    /// View this `Offset` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consumes this `Offset`, and returns its underlying [`UnsignedInt`] value.
    pub fn into_unsigned_int(self) -> UnsignedInt {
        self.0
    }

    /// Parses a `Offset` from a UTF-8 `str` representing a 64-bit integer.
    pub fn from_str_u64(s: &str) -> Result<Offset, ParserError> {
        UnsignedInt::from_str_u64(s)
            .map(Offset::from)
            .map_err(|e| ParserError::Offset(e.to_string()))
    }

    /// Parses a `Offset` from a UTF-8 `str` representing a 32-bit integer.
    pub fn from_str_u32(s: &str) -> Result<Offset, ParserError> {
        UnsignedInt::from_str_u32(s)
            .map(Offset::from)
            .map_err(|e| ParserError::Offset(e.to_string()))
    }

    /// Converts a byte string to a 64-bit `Offset`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u64<T>(bytes: T) -> Result<Offset, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        UnsignedInt::try_from_u64(bytes).map(Self)
    }

    /// Converts a byte string to a 32-bit `Offset`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u32<T>(bytes: T) -> Result<Offset, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        UnsignedInt::try_from_u32(bytes).map(Self)
    }

    /// Returns the underlying `u64` in this `Offset` if applicable, `None` otherwise.
    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    /// Returns the underlying `u32` in this `Offset` if applicable, `None` otherwise.
    pub fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }
}

impl AsRef<Offset> for Offset {
    #[inline]
    fn as_ref(&self) -> &Offset {
        self
    }
}

impl AsRef<UnsignedInt> for Offset {
    #[inline]
    fn as_ref(&self) -> &UnsignedInt {
        self.0.as_ref()
    }
}

impl AsRef<str> for Offset {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<UnsignedInt> for Offset {
    #[inline]
    fn from(num: UnsignedInt) -> Offset {
        Self(num)
    }
}

impl From<u32> for Offset {
    #[inline]
    fn from(value: u32) -> Offset {
        Self(UnsignedInt::from(value))
    }
}

impl From<u64> for Offset {
    #[inline]
    fn from(value: u64) -> Offset {
        Self(UnsignedInt::from(value))
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
