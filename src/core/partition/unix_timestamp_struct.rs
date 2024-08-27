// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Number of seconds since Jan. 1, 1970.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnixTimestamp {
    ts: u64,
    ts_str: String,
}

impl UnixTimestamp {
    /// View this `UnixTimestamp` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.ts_str
    }
}

impl AsRef<UnixTimestamp> for UnixTimestamp {
    #[inline]
    fn as_ref(&self) -> &UnixTimestamp {
        self
    }
}

impl AsRef<str> for UnixTimestamp {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.ts_str
    }
}

impl fmt::Display for UnixTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.ts_str)
    }
}

impl From<u64> for UnixTimestamp {
    #[inline]
    fn from(ts: u64) -> UnixTimestamp {
        Self {
            ts,
            ts_str: ts.to_string(),
        }
    }
}

impl TryFrom<&[u8]> for UnixTimestamp {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::UnixTimestamp(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str(s).map_err(|e| ConversionError::UnixTimestamp(e.to_string()))
            })
    }
}

impl TryFrom<Vec<u8>> for UnixTimestamp {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for UnixTimestamp {
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
                .ok_or(ParserError::UnixTimestamp(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::UnixTimestamp(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let ts = u64::from_str(stripped).map_err(|e| {
            let err_msg = format!("invalid integer value: {:?} {}", s, e);
            ParserError::UnixTimestamp(err_msg)
        })?;

        Ok(Self::from(ts))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn unix_timestamp_can_not_parse_an_empty_string() {
        let _: UnixTimestamp = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn unix_timestamp_can_not_parse_an_invalid_time_stamp() {
        let _: UnixTimestamp = "timestamp".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn unix_timestamp_can_not_parse_a_timestamp_with_an_unclosed_double_quote() {
        let _: UnixTimestamp = r#""2"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn unix_timestamp_can_not_parse_a_timestamp_with_an_unclosed_quote() {
        let _: UnixTimestamp = "'2".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn unix_timestamp_can_not_convert_invalid_bytes_into_a_unix_timestamp() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = UnixTimestamp::try_from(bytes).unwrap();
    }

    #[test]
    fn unix_timestamp_can_convert_valid_bytes_into_a_unix_timestamp() -> crate::Result<()> {
        let bytes: Vec<u8> = b"1724850577".to_vec();
        let actual = UnixTimestamp::try_from(bytes)?;
        let ts_str = "1724850577";
        let expected = UnixTimestamp {
            ts: 1724850577u64,
            ts_str: ts_str.to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn unix_timestamp_can_parse_a_valid_time_stamp() -> crate::Result<()> {
        let ts_str = "1724850577";
        let actual: UnixTimestamp = ts_str.parse()?;
        let expected = UnixTimestamp {
            ts: 1724850577u64,
            ts_str: ts_str.to_string(),
        };
        assert_eq!(actual, expected);

        let ts_str = r#""1724850577""#;
        let actual: UnixTimestamp = ts_str.parse()?;
        let expected = UnixTimestamp {
            ts: 1724850577u64,
            ts_str: ts_str.trim_matches('"').to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }
}
