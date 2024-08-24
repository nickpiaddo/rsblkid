// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// A device label.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Label(String);

impl Label {
    /// View this `Label` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Label> for Label {
    #[inline]
    fn as_ref(&self) -> &Label {
        self
    }
}

impl AsRef<str> for Label {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Put label between quotes if it contains special characters.
        let label = if self
            .0
            .chars()
            .any(|c| c.is_whitespace() || c.is_ascii_punctuation())
        {
            format!("\"{}\"", self.0)
        } else {
            self.0.to_owned()
        };

        write!(f, "{label}")
    }
}

impl TryFrom<&[u8]> for Label {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(bytes.to_vec())
    }
}

impl TryFrom<Vec<u8>> for Label {
    type Error = ConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        String::from_utf8(bytes).map(Label).map_err(|e| {
            ConversionError::Label(format!("bytes to UTF-8 string conversion error. {:?}", e))
        })
    }
}

impl FromStr for Label {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove opening opening/closing quotes/double-quotes if present
        let err_missing_dquote = format!("missing closing double-quote in: {}", s);
        let err_missing_quote = format!("missing closing quote in: {}", s);

        let trimmed = s.trim();
        let parsed = if trimmed.starts_with('"') {
            trimmed
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or(ParserError::Label(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::Label(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let label = Self(parsed.to_owned());

        Ok(label)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn label_can_not_parse_a_label_string_with_an_unclosed_double_quote() {
        let _: Label = r#""082"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn label_can_not_parse_a_label_string_with_an_unclosed_quote() {
        let _: Label = "'082".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string conversion error")]
    fn label_can_not_convert_invalid_bytes_into_a_label() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = Label::try_from(bytes).unwrap();
    }

    #[test]
    fn label_can_convert_valid_bytes_into_a_label() -> crate::Result<()> {
        let bytes: Vec<u8> = vec![240, 159, 146, 150];
        let actual = Label::try_from(bytes)?;
        let label = String::from("💖");
        let expected = Label(label);
        assert_eq!(actual, expected);

        let bytes: Vec<u8> = b"hello, world".to_vec();
        let actual = Label::try_from(bytes)?;
        let label = String::from("hello, world");
        let expected = Label(label);
        assert_eq!(actual, expected);

        Ok(())
    }
}
