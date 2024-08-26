// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Unsigned integer restricted to a `u32`, or `u64`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum UnsignedInt {
    U32(u32, String),
    U64(u64, String),
}

impl UnsignedInt {
    /// View this `UnsignedInt` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::U32(_, ref s) => s,
            Self::U64(_, ref s) => s,
        }
    }

    /// Parses an `UnsignedInt::U64` from a UTF-8 `str`.
    pub fn from_str_u64(s: &str) -> Result<UnsignedInt, ParserError> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let stripped = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::UnsignedInt(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::UnsignedInt(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let num = u64::from_str(stripped).map_err(|e| {
            let err_msg = format!("invalid integer value: {:?} in {:?} {}", stripped, s, e);
            ParserError::UnsignedInt(err_msg)
        })?;

        Ok(Self::from(num))
    }

    /// Parses an `UnsignedInt::U32` from a UTF-8 `str`.
    pub fn from_str_u32(s: &str) -> Result<UnsignedInt, ParserError> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let stripped = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::UnsignedInt(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::UnsignedInt(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let num = u32::from_str(stripped).map_err(|e| {
            let err_msg = format!("invalid integer value: {:?} in {:?} {}", stripped, s, e);
            ParserError::UnsignedInt(err_msg)
        })?;

        Ok(Self::from(num))
    }

    /// Converts a byte string to a 64-bit `UnsignedInt`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u64<T>(bytes: T) -> Result<UnsignedInt, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();

        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::UnsignedInt(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str_u64(s).map_err(|e| ConversionError::UnsignedInt(e.to_string()))
            })
    }

    /// Converts a byte string to a 32-bit `UnsignedInt`. The byte string contains a string
    /// representation of an integer.
    pub fn try_from_u32<T>(bytes: T) -> Result<UnsignedInt, ConversionError>
    where
        T: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();

        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::UnsignedInt(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str_u32(s).map_err(|e| ConversionError::UnsignedInt(e.to_string()))
            })
    }

    /// Returns the underlying `u64` in this `UnsignedInt` if applicable, `None` otherwise.
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            Self::U32(_, _) => None,
            Self::U64(value, _) => Some(*value),
        }
    }

    /// Returns the underlying `u32` in this `UnsignedInt` if applicable, `None` otherwise.
    pub fn to_u32(&self) -> Option<u32> {
        match self {
            Self::U64(_, _) => None,
            Self::U32(value, _) => Some(*value),
        }
    }
}

impl AsRef<UnsignedInt> for UnsignedInt {
    #[inline]
    fn as_ref(&self) -> &UnsignedInt {
        self
    }
}

impl From<u32> for UnsignedInt {
    #[inline]
    fn from(value: u32) -> UnsignedInt {
        UnsignedInt::U32(value, value.to_string())
    }
}

impl From<u64> for UnsignedInt {
    #[inline]
    fn from(value: u64) -> UnsignedInt {
        UnsignedInt::U64(value, value.to_string())
    }
}

impl fmt::Display for UnsignedInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn unsigned_int_from_str_u64_can_not_parse_an_unsigned_int_string_with_an_unclosed_double_quote(
    ) {
        let s = r#""1234"#;
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn unsigned_int_from_str_u64_can_not_parse_an_unsigned_int_string_with_an_unclosed_quote() {
        let s = "'1234";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn unsigned_int_from_str_u32_can_not_parse_an_unsigned_int_string_with_an_unclosed_double_quote(
    ) {
        let s = r#""1234"#;
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn unsigned_int_from_str_u32_can_not_parse_an_unsigned_int_string_with_an_unclosed_quote() {
        let s = "'1234";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn unsigned_int_from_str_u64_can_not_parse_an_invalid_unsigned_int_type() {
        let s = "DUMMY";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn unsigned_int_from_str_u32_can_not_parse_an_invalid_unsigned_int_type() {
        let s = "DUMMY";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "number too large to fit in target type")]
    fn unsigned_int_from_str_u64_can_not_parse_an_unsigned_int_larger_than_max_u64() {
        let s = "118446744073709551615";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "number too large to fit in target type")]
    fn unsigned_int_from_str_u32_can_not_parse_an_unsigned_int_larger_than_max_u32() {
        let s = "118446744073709551615";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u64_can_not_parse_a_negative_integer() {
        let s = "-42";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u32_can_not_parse_a_negative_integer() {
        let s = "-42";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u64_can_not_parse_a_float() {
        let s = "42.0";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u32_can_not_parse_a_float() {
        let s = "42.0";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u64_can_not_parse_a_negative_float() {
        let s = "-42.0";
        let _ = UnsignedInt::from_str_u64(s).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    fn unsigned_int_from_str_u32_can_not_parse_a_negative_float() {
        let s = "-42.0";
        let _ = UnsignedInt::from_str_u32(s).unwrap();
    }

    #[test]
    fn unsigned_int_can_parse_a_valid_integer() -> crate::Result<()> {
        let s = r#""11844674""#;
        let actual = UnsignedInt::from_str_u64(s)?;
        let integer = 11844674u64;
        let expected = UnsignedInt::U64(integer, integer.to_string());
        assert_eq!(actual, expected);

        let s = r#""11844674""#;
        let actual = UnsignedInt::from_str_u32(s)?;
        let integer = 11844674u32;
        let expected = UnsignedInt::U32(integer, integer.to_string());
        assert_eq!(actual, expected);

        let s = "'11844674'";
        let actual = UnsignedInt::from_str_u64(s)?;
        let integer = 11844674u64;
        let expected = UnsignedInt::U64(integer, integer.to_string());
        assert_eq!(actual, expected);

        let s = "'11844674'";
        let actual = UnsignedInt::from_str_u32(s)?;
        let integer = 11844674u32;
        let expected = UnsignedInt::U32(integer, integer.to_string());
        assert_eq!(actual, expected);

        let s = "11844674";
        let actual = UnsignedInt::from_str_u64(s)?;
        let integer = 11844674u64;
        let expected = UnsignedInt::U64(integer, integer.to_string());
        assert_eq!(actual, expected);

        let s = "11844674";
        let actual = UnsignedInt::from_str_u32(s)?;
        let integer = 11844674u32;
        let expected = UnsignedInt::U32(integer, integer.to_string());
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn unsigned_int_try_from_u64_can_not_convert_invalid_bytes_into_an_unsigned_int() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = UnsignedInt::try_from_u64(bytes).unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn unsigned_int_try_from_u32_can_not_convert_invalid_bytes_into_an_unsigned_int() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = UnsignedInt::try_from_u32(bytes).unwrap();
    }

    #[test]
    #[should_panic(expected = "number too large to fit in target type")]
    fn unsigned_int_try_from_u64_can_not_convert_bytes_larger_than_max_u64() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = b"118446744073709551615".to_vec();
        let _ = UnsignedInt::try_from_u64(bytes).unwrap();
    }

    #[test]
    #[should_panic(expected = "number too large to fit in target type")]
    fn unsigned_int_try_from_u32_can_not_convert_bytes_larger_than_max_u32() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = b"118446744073709551615".to_vec();
        let _ = UnsignedInt::try_from_u32(bytes).unwrap();
    }

    #[test]
    fn unsigned_int_can_convert_valid_bytes_into_an_u64_unsigned_int() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0".to_vec();
        let actual = UnsignedInt::try_from_u64(bytes)?;
        let integer = 0u64;
        let expected = UnsignedInt::U64(integer, integer.to_string());
        assert_eq!(actual, expected);

        let bytes: Vec<u8> = b"18446744073709551615".to_vec();
        let actual = UnsignedInt::try_from_u64(bytes)?;
        let integer = u64::MAX;
        let expected = UnsignedInt::U64(integer, integer.to_string());
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn unsigned_int_can_convert_valid_bytes_into_an_u32_unsigned_int() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0".to_vec();
        let actual = UnsignedInt::try_from_u32(bytes)?;
        let integer = 0u32;
        let expected = UnsignedInt::U32(integer, integer.to_string());
        assert_eq!(actual, expected);

        let bytes: Vec<u8> = b"4294967295".to_vec();
        let actual = UnsignedInt::try_from_u32(bytes)?;
        let integer = u32::MAX;
        let expected = UnsignedInt::U32(integer, integer.to_string());
        assert_eq!(actual, expected);

        Ok(())
    }
}
