// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

use crate::core::num::UnsignedInt;

/// Size of a device, partition, etc.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size(UnsignedInt);

impl Size {
    /// Returns the `Size` value.
    pub fn value(&self) -> &UnsignedInt {
        &self.0
    }

    /// View this `Size` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consumes this `Size`, and returns its underlying [`UnsignedInt`] value.
    pub fn into_unsigned_int(self) -> UnsignedInt {
        self.0
    }

    /// Parses a `Size` from a UTF-8 `str` representing a 64-bit integer.
    pub fn from_str_u64(s: &str) -> Result<Size, ParserError> {
        UnsignedInt::from_str_u64(s)
            .map(Size::from)
            .map_err(|e| ParserError::Size(e.to_string()))
    }

    /// Parses a `Size` from a UTF-8 `str` representing a 32-bit integer.
    pub fn from_str_u32(s: &str) -> Result<Size, ParserError> {
        UnsignedInt::from_str_u32(s)
            .map(Size::from)
            .map_err(|e| ParserError::Size(e.to_string()))
    }

    /// Converts a byte string to a 64-bit `Size`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u64<T>(bytes: T) -> Result<Size, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        UnsignedInt::try_from_u64(bytes).map(Self)
    }

    /// Converts a byte string to a 32-bit `Size`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u32<T>(bytes: T) -> Result<Size, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        UnsignedInt::try_from_u32(bytes).map(Self)
    }

    /// Returns the underlying `u64` in this `Size` if applicable, `None` otherwise.
    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    /// Returns the underlying `u32` in this `Size` if applicable, `None` otherwise.
    pub fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }
}

impl AsRef<Size> for Size {
    #[inline]
    fn as_ref(&self) -> &Size {
        self
    }
}

impl AsRef<UnsignedInt> for Size {
    #[inline]
    fn as_ref(&self) -> &UnsignedInt {
        self.0.as_ref()
    }
}

impl AsRef<str> for Size {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<UnsignedInt> for Size {
    #[inline]
    fn from(num: UnsignedInt) -> Size {
        Self(num)
    }
}

impl From<u32> for Size {
    #[inline]
    fn from(value: u32) -> Size {
        Self(UnsignedInt::from(value))
    }
}

impl From<u64> for Size {
    #[inline]
    fn from(value: u64) -> Size {
        Self(UnsignedInt::from(value))
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
