// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// A device's identification number.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceNumber {
    dev_num: u64,
    dev_num_str: String,
}

impl DeviceNumber {
    /// View this `DeviceNumber` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        &self.dev_num_str
    }
}

impl AsRef<DeviceNumber> for DeviceNumber {
    #[inline]
    fn as_ref(&self) -> &DeviceNumber {
        self
    }
}

impl AsRef<str> for DeviceNumber {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.dev_num_str
    }
}

impl fmt::Display for DeviceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.dev_num_str)
    }
}

impl From<u64> for DeviceNumber {
    #[inline]
    fn from(dev_num: u64) -> DeviceNumber {
        Self {
            dev_num,
            dev_num_str: dev_num.to_string(),
        }
    }
}

impl TryFrom<&[u8]> for DeviceNumber {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::DeviceNumber(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str(s).map_err(|e| ConversionError::DeviceNumber(e.to_string()))
            })
    }
}

impl TryFrom<Vec<u8>> for DeviceNumber {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for DeviceNumber {
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
                .ok_or(ParserError::DeviceNumber(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::DeviceNumber(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let dev_num = u64::from_str(stripped).map_err(|e| {
            let err_msg = format!("invalid integer value: {:?} {}", s, e);
            ParserError::DeviceNumber(err_msg)
        })?;

        Ok(Self::from(dev_num))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn device_number_can_not_parse_an_empty_string() {
        let _: DeviceNumber = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid integer value")]
    fn device_number_can_not_parse_an_invalid_device_number() {
        let _: DeviceNumber = "device_number".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn device_number_can_not_parse_a_device_number_with_an_unclosed_double_quote() {
        let _: DeviceNumber = r#""2"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn device_number_can_not_parse_a_device_number_with_an_unclosed_quote() {
        let _: DeviceNumber = "'2".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn device_number_can_not_convert_invalid_bytes_into_a_device_number() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = DeviceNumber::try_from(bytes).unwrap();
    }

    #[test]
    fn device_number_can_convert_valid_bytes_into_a_device_number() -> crate::Result<()> {
        let bytes: Vec<u8> = b"1724850577".to_vec();
        let actual = DeviceNumber::try_from(bytes)?;
        let dev_num_str = "1724850577";
        let expected = DeviceNumber {
            dev_num: 1724850577u64,
            dev_num_str: dev_num_str.to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn device_number_can_parse_a_valid_device_number() -> crate::Result<()> {
        let dev_num_str = "1724850577";
        let actual: DeviceNumber = dev_num_str.parse()?;
        let expected = DeviceNumber {
            dev_num: 1724850577u64,
            dev_num_str: dev_num_str.to_string(),
        };
        assert_eq!(actual, expected);

        let dev_num_str = r#""1724850577""#;
        let actual: DeviceNumber = dev_num_str.parse()?;
        let expected = DeviceNumber {
            dev_num: 1724850577u64,
            dev_num_str: dev_num_str.trim_matches('"').to_string(),
        };
        assert_eq!(actual, expected);

        Ok(())
    }
}
