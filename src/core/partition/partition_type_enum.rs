// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

use crate::core::partition::Guid;
use crate::core::partition::OSType;

/// Supported partition types.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[non_exhaustive]
pub enum PartitionType {
    MBR(OSType),
    GPT(Guid),
}

impl PartitionType {
    /// View this `PartitionType` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::MBR(value) => value.as_str(),
            Self::GPT(value) => value.as_str(),
        }
    }
}

impl AsRef<PartitionType> for PartitionType {
    #[inline]
    fn as_ref(&self) -> &PartitionType {
        self
    }
}

impl AsRef<str> for PartitionType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PartitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for PartitionType {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        OSType::try_from(bytes)
            .map(Self::MBR)
            .or_else(|_| Guid::try_from(bytes).map(Self::GPT))
            .map_err(|_| ConversionError::PartitionType("error converting bytes".to_owned()))
    }
}

impl TryFrom<Vec<u8>> for PartitionType {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for PartitionType {
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
                .ok_or(ParserError::PartitionType(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::PartitionType(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        OSType::from_str(stripped)
            .map(Self::MBR)
            .or_else(|_| Guid::from_str(stripped).map(Self::GPT))
            .map_err(|_| ParserError::PartitionType(format!("unsupported partition type: {}", s)))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn partition_type_can_not_parse_an_os_type_string_with_an_unclosed_double_quote() {
        let _: PartitionType = r#""0x82"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn partition_type_can_not_parse_an_os_type_string_with_an_unclosed_quote() {
        let _: PartitionType = "'0x82".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "error converting bytes")]
    fn partition_type_can_not_convert_invalid_bytes_into_an_mbr_os_type() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = PartitionType::try_from(bytes).unwrap();
    }

    #[test]
    fn partition_type_can_convert_valid_bytes_into_an_mbr_os_type() -> crate::Result<()> {
        let bytes: Vec<u8> = b"0x83".to_vec();
        let actual = PartitionType::try_from(bytes)?;
        let expected = PartitionType::MBR(OSType::Linux);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_type_can_convert_valid_bytes_into_a_gpt_guid() -> crate::Result<()> {
        let bytes: Vec<u8> = b"6a85cf4d-1dd2-11b2-99a6-080020736631".to_vec();
        let actual = PartitionType::try_from(bytes)?;
        let expected = PartitionType::GPT(Guid::SolarisRoot);
        assert_eq!(actual, expected);

        Ok(())
    }
}
