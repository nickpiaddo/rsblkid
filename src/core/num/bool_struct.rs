// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// A boolean value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bool {
    state: bool,
    state_str: String,
}

impl Bool {
    /// View this `Bool` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.state_str
    }
}

impl AsRef<Bool> for Bool {
    #[inline]
    fn as_ref(&self) -> &Bool {
        self
    }
}

impl AsRef<str> for Bool {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.state_str
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.state_str)
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(state: bool) -> Bool {
        let state_str = match state {
            true => "1".to_owned(),
            false => "0".to_owned(),
        };

        Self { state, state_str }
    }
}

impl TryFrom<&[u8]> for Bool {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::Bool(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| Self::from_str(s).map_err(|e| ConversionError::Bool(e.to_string())))
    }
}

impl TryFrom<Vec<u8>> for Bool {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for Bool {
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
                .ok_or(ParserError::Bool(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Bool(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let state = match stripped.trim() {
            "1" => Ok(true),
            "0" => Ok(false),
            _otherwise => {
                let err_msg = format!("invalid boolean value: {:?}. Expected 0 or 1", s);

                Err(ParserError::Bool(err_msg))
            }
        }?;

        Ok(Self::from(state))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "Expected 0 or 1")]
    fn bool_can_not_parse_an_empty_string() {
        let _: Bool = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn bool_can_not_parse_a_boolean_string_with_an_unclosed_double_quote() {
        let _: Bool = r#""1"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn bool_can_not_parse_a_boolean_string_with_an_unclosed_quote() {
        let _: Bool = "'1".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected 0 or 1")]
    fn bool_can_not_parse_an_invalid_bool_type() {
        let _: Bool = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn bool_can_not_convert_invalid_bytes_into_a_boolean() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Bool::try_from(bytes).unwrap();
    }

    #[test]
    fn bool_can_convert_valid_bytes_into_a_boolean() -> crate::Result<()> {
        let bytes: Vec<u8> = b"1".to_vec();
        let actual = Bool::try_from(bytes)?;
        let bool = "1";
        let expected = Bool {
            state: true,
            state_str: bool.to_string(),
        };
        assert_eq!(actual, expected);

        let bytes: Vec<u8> = b"0".to_vec();
        let actual = Bool::try_from(bytes)?;
        let bool = "0";
        let expected = Bool {
            state: false,
            state_str: bool.to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }
}
