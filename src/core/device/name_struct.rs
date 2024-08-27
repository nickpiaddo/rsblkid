// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// A device's name.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Name(String);

impl Name {
    /// View this `Name` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Name> for Name {
    #[inline]
    fn as_ref(&self) -> &Name {
        self
    }
}

impl AsRef<str> for Name {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl TryFrom<&[u8]> for Name {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(bytes.to_vec())
    }
}

impl TryFrom<Vec<u8>> for Name {
    type Error = ConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        String::from_utf8(bytes).map(Name).map_err(|e| {
            ConversionError::Name(format!("bytes to UTF-8 string conversion error. {:?}", e))
        })
    }
}

impl FromStr for Name {
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
                .ok_or(ParserError::Name(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Name(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        Ok(Self(stripped.to_owned()))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn name_can_not_parse_a_name_string_with_an_unclosed_double_quote() {
        let _: Name = r#""082"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn name_can_not_parse_a_name_string_with_an_unclosed_quote() {
        let _: Name = "'082".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string conversion error")]
    fn name_can_not_convert_invalid_bytes_into_a_name() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Name::try_from(bytes).unwrap();
    }

    #[test]
    fn name_can_convert_valid_bytes_into_a_name() -> crate::Result<()> {
        let bytes: Vec<u8> = b"Backup".to_vec();
        let actual = Name::try_from(bytes)?;
        let name = String::from("Backup");
        let expected = Name(name);
        assert_eq!(actual, expected);

        Ok(())
    }
}
