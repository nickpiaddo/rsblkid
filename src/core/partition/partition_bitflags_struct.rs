// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Bit flags in partition entries.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PartitionBitflags {
    bitflags: u64,
    bitflags_str: String,
}

impl PartitionBitflags {
    /// View this `PartitionBitflags` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.bitflags_str
    }
}

impl AsRef<PartitionBitflags> for PartitionBitflags {
    #[inline]
    fn as_ref(&self) -> &PartitionBitflags {
        self
    }
}

impl AsRef<str> for PartitionBitflags {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.bitflags_str
    }
}

impl fmt::Display for PartitionBitflags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.bitflags_str)
    }
}

impl From<u64> for PartitionBitflags {
    #[inline]
    fn from(bitflags: u64) -> PartitionBitflags {
        Self {
            bitflags,
            bitflags_str: format!("{:#018x}", bitflags),
        }
    }
}

impl TryFrom<&[u8]> for PartitionBitflags {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::PartitionBitflags(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str(s).map_err(|e| ConversionError::PartitionBitflags(e.to_string()))
            })
    }
}

impl TryFrom<Vec<u8>> for PartitionBitflags {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for PartitionBitflags {
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
                .ok_or(ParserError::PartitionBitflags(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::PartitionBitflags(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        // Remove hex string prefix and convert to `OSType`.
        stripped
            .trim()
            .strip_prefix("0x")
            .ok_or(ParserError::PartitionBitflags(format!(
                "missing '0x' prefix in: {}",
                s
            )))
            .and_then(|h| {
                u64::from_str_radix(h, 16).map(Self::from).map_err(|e| {
                    let err_msg = format!("invalid hexadecimal value: {:?} {}", s, e);
                    ParserError::PartitionBitflags(err_msg)
                })
            })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn partition_bitflags_can_not_parse_bitflags_with_an_unclosed_double_quote() {
        let _: PartitionBitflags = r#""0x0123456789abcdef"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn partition_bitflags_can_not_parse_bitflags_with_an_unclosed_quote() {
        let _: PartitionBitflags = "'0x0123456789abcdef".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing '0x' prefix")]
    fn partition_bitflags_can_not_parse_an_empty_string() {
        let _: PartitionBitflags = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid hexadecimal value")]
    fn partition_bitflags_can_not_parse_invalid_bitflags() {
        let _: PartitionBitflags = "0xbitflags".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn partition_bitflags_can_not_convert_invalid_bytes_into_bitflags() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = PartitionBitflags::try_from(bytes).unwrap();
    }

    #[test]
    fn partition_bitflags_can_convert_valid_bytes_into_bitflags() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0x0123456789abcdef".to_vec();
        let actual = PartitionBitflags::try_from(bytes)?;
        let bitflags = "0x0123456789abcdef";
        let expected = PartitionBitflags {
            bitflags: 81985529216486895u64,
            bitflags_str: bitflags.to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_bitflags_can_parse_valid_bitflags() -> crate::Result<()> {
        let bitflags = "0x0123456789abcdef";
        let actual: PartitionBitflags = bitflags.parse()?;
        let expected = PartitionBitflags {
            bitflags: 81985529216486895u64,
            bitflags_str: bitflags.to_string(),
        };
        assert_eq!(actual, expected);

        let bitflags = r#""0x0123456789abcdef""#;
        let actual: PartitionBitflags = bitflags.parse()?;
        let expected = PartitionBitflags {
            bitflags: 81985529216486895u64,
            bitflags_str: bitflags.trim_matches('"').to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }
}
