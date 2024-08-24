// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use num_enum::IntoPrimitive;

// From standard library
use std::ffi::CString;
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Device usage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, IntoPrimitive)]
#[non_exhaustive]
#[repr(i32)]
pub enum Usage {
    FileSystem = libblkid::BLKID_USAGE_FILESYSTEM,
    Raid = libblkid::BLKID_USAGE_RAID,
    Crypto = libblkid::BLKID_USAGE_CRYPTO,
    Other = libblkid::BLKID_USAGE_OTHER,
    Unknown = 0i32,
}

impl Usage {
    /// View this `Usage` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::FileSystem => "filesystem",
            Self::Raid => "raid",
            Self::Crypto => "crypto",
            Self::Other => "other",
            Self::Unknown => "unknown",
        }
    }

    /// Converts this `Usage` to a [`CString`].
    pub fn to_c_string(&self) -> CString {
        CString::new(self.as_str()).unwrap()
    }
}

impl AsRef<Usage> for Usage {
    #[inline]
    fn as_ref(&self) -> &Usage {
        self
    }
}

impl AsRef<str> for Usage {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for Usage {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::Usage(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::Usage(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for Usage {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for Usage {
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
                .ok_or(ParserError::Usage(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Usage(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        match stripped.trim().to_lowercase().as_str() {
            "filesystem" => Ok(Self::FileSystem),
            "raid" => Ok(Self::Raid),
            "crypto" => Ok(Self::Crypto),
            "other" => Ok(Self::Other),
            "unknown" => Ok(Self::Unknown),
            _unsupported => {
                let err_msg = format!("unsupported device usage: {:?}", s);
                Err(ParserError::Usage(err_msg))
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
    #[should_panic(expected = "unsupported device usage")]
    fn usage_can_not_parse_an_empty_string() {
        let _: Usage = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn usage_can_not_parse_a_usage_string_with_an_unclosed_double_quote() {
        let _: Usage = r#""filesystem"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn usage_can_not_parse_a_usage_string_with_an_unclosed_quote() {
        let _: Usage = "'filesystem".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "unsupported device usage")]
    fn usage_can_not_parse_an_invalid_device_usage() {
        let _: Usage = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn usage_can_not_convert_invalid_bytes_into_a_device_usage() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Usage::try_from(bytes).unwrap();
    }

    #[test]
    fn usage_can_convert_valid_bytes_into_a_device_usage() -> crate::Result<()> {
        let bytes: Vec<u8> = b"crypto".to_vec();
        let actual = Usage::try_from(bytes)?;
        let expected = Usage::Crypto;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn usage_can_parse_a_valid_device_usage() -> crate::Result<()> {
        let usage_str = "filesystem";
        let actual: Usage = usage_str.parse()?;
        let expected = Usage::FileSystem;
        assert_eq!(actual, expected);

        let usage_str = "raid";
        let actual: Usage = usage_str.parse()?;
        let expected = Usage::Raid;
        assert_eq!(actual, expected);

        let usage_str = "crypto";
        let actual: Usage = usage_str.parse()?;
        let expected = Usage::Crypto;
        assert_eq!(actual, expected);

        let usage_str = "unknown";
        let actual: Usage = usage_str.parse()?;
        let expected = Usage::Unknown;
        assert_eq!(actual, expected);

        Ok(())
    }
}
