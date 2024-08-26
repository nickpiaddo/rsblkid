// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Data endianness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum Endian {
    Big,
    Little,
}

impl Endian {
    /// View this `Endian` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Big => "BIG",
            Self::Little => "LITTLE",
        }
    }
}

impl AsRef<Endian> for Endian {
    #[inline]
    fn as_ref(&self) -> &Endian {
        self
    }
}

impl AsRef<str> for Endian {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for Endian {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::Endian(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::Endian(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for Endian {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for Endian {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let stripped = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::Endian(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Endian(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        match stripped.trim().to_uppercase().as_str() {
            "BIG" => Ok(Self::Big),
            "LITTLE" => Ok(Self::Little),
            _unsupported => {
                let err_msg = format!("unsupported endianness value: {:?}", s);
                Err(ParserError::Endian(err_msg))
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "unsupported endianness value")]
    fn endian_can_not_parse_empty_string() {
        let _: Endian = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn endian_can_not_parse_an_endian_string_with_an_unclosed_double_quote() {
        let _: Endian = r#""BIG"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn endian_can_not_parse_an_endian_string_with_an_unclosed_quote() {
        let _: Endian = "'BIG".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "unsupported endianness value")]
    fn endian_can_not_parse_an_invalid_endian() {
        let _: Endian = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn endian_can_not_convert_invalid_bytes_into_an_endian() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Endian::try_from(bytes).unwrap();
    }

    #[test]
    fn endian_can_convert_valid_bytes_into_an_endian() -> crate::Result<()> {
        let bytes: Vec<u8> = b"LITTLE".to_vec();
        let actual = Endian::try_from(bytes)?;
        let expected = Endian::Little;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn endian_can_parse_big_endian() -> crate::Result<()> {
        let actual: Endian = "BIG".parse()?;
        let expected = Endian::Big;
        assert_eq!(actual, expected);

        let actual: Endian = "big".parse()?;
        let expected = Endian::Big;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn endian_can_parse_little_endian() -> crate::Result<()> {
        let actual: Endian = "LITTLE".parse()?;
        let expected = Endian::Little;
        assert_eq!(actual, expected);

        let actual: Endian = "little".parse()?;
        let expected = Endian::Little;
        assert_eq!(actual, expected);
        Ok(())
    }
}
