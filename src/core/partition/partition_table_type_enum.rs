// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use enum_iterator::Sequence;

// From standard library
use std::ffi::CString;
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ConversionError;
use crate::core::errors::ParserError;

/// Supported partition tables.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Sequence)]
#[non_exhaustive]
pub enum PartitionTableType {
    /// Name: `"aix"`
    AIX,
    /// Name: `"atari"`
    Atari,
    /// Name: `"bsd"`
    BSD,
    /// Name: `"dos"`
    DOS,
    /// Name: `"freebsd"`
    FreeBSD,
    /// Name: `"gpt"`
    GPT,
    /// Name: `"mac"`
    Mac,
    /// Name: `"minix"`
    Minix,
    /// Name: `"netbsd"`
    NetBSD,
    /// Name: `"openbsd"`
    OpenBSD,
    /// Name: `"PMBR"`
    ProtectiveMBR,
    /// Name: `"sgi"`
    SGI,
    /// Name: `"solaris"`
    SolarisX86,
    /// Name: `"sun"`
    Sun,
    /// Name: `"ultrix"`
    Ultrix,
    /// Name: `"unixware"`
    Unixware,
}

impl PartitionTableType {
    /// View this `PartitionTableType` as a UTF-8 `str`.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AIX => "aix",
            Self::Atari => "atari",
            Self::BSD => "bsd",
            Self::DOS => "dos",
            Self::FreeBSD => "freebsd",
            Self::GPT => "gpt",
            Self::Mac => "mac",
            Self::Minix => "minix",
            Self::NetBSD => "netbsd",
            Self::OpenBSD => "openbsd",
            Self::ProtectiveMBR => "PMBR",
            Self::SGI => "sgi",
            Self::SolarisX86 => "solaris",
            Self::Sun => "sun",
            Self::Ultrix => "ultrix",
            Self::Unixware => "unixware",
        }
    }

    /// Converts this `PartitionTableType` to a [`CString`].
    pub fn to_c_string(&self) -> CString {
        // PartitionTableType's string representation does not contain NULL characters,  we can
        // safely unwrap the new CString.
        CString::new(self.as_str()).unwrap()
    }
}

impl AsRef<PartitionTableType> for PartitionTableType {
    #[inline]
    fn as_ref(&self) -> &PartitionTableType {
        self
    }
}

impl AsRef<str> for PartitionTableType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PartitionTableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for PartitionTableType {
    type Error = ConversionError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes)
            .map_err(|e| {
                ConversionError::PartitionTableType(format!(
                    "bytes to UTF-8 string slice conversion error. {:?}",
                    e
                ))
            })
            .and_then(|s| {
                Self::from_str(s).map_err(|e| ConversionError::PartitionTableType(e.to_string()))
            })
    }
}

impl TryFrom<Vec<u8>> for PartitionTableType {
    type Error = ConversionError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl FromStr for PartitionTableType {
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
                .ok_or(ParserError::PartitionTableType(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::PartitionTableType(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        match stripped.trim() {
            "aix" => Ok(Self::AIX),
            "atari" => Ok(Self::Atari),
            "bsd" => Ok(Self::BSD),
            "dos" => Ok(Self::DOS),
            "freebsd" => Ok(Self::FreeBSD),
            "gpt" => Ok(Self::GPT),
            "mac" => Ok(Self::Mac),
            "minix" => Ok(Self::Minix),
            "netbsd" => Ok(Self::NetBSD),
            "openbsd" => Ok(Self::OpenBSD),
            "PMBR" => Ok(Self::ProtectiveMBR),
            "sgi" => Ok(Self::SGI),
            "solaris" => Ok(Self::SolarisX86),
            "sun" => Ok(Self::Sun),
            "ultrix" => Ok(Self::Ultrix),
            "unixware" => Ok(Self::Unixware),
            _unsupported => {
                let err_msg = format!("unsupported partition type: {:?}", s);

                Err(ParserError::PartitionTableType(err_msg))
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
    #[should_panic(expected = "unsupported partition type")]
    fn partition_table_type_can_not_parse_an_empty_string() {
        let _: PartitionTableType = "".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing double-quote")]
    fn partition_table_type_can_not_parse_a_partition_table_type_string_with_an_unclosed_double_quote(
    ) {
        let _: PartitionTableType = r#""atari"#.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "missing closing quote")]
    fn partition_table_type_can_not_parse_a_partition_table_type_string_with_an_unclosed_quote() {
        let _: PartitionTableType = "'atari".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "unsupported partition type")]
    fn partition_table_type_can_not_parse_an_invalid_type() {
        let _: PartitionTableType = "DUMMY".parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "bytes to UTF-8 string slice conversion error")]
    fn partition_table_type_can_not_convert_invalid_bytes_into_a_partition_table_type() {
        // some invalid bytes, in a vector
        let bytes: Vec<u8> = vec![0, 159, 146, 150];
        let _ = PartitionTableType::try_from(bytes).unwrap();
    }

    #[test]
    fn partition_table_type_can_convert_valid_bytes_into_a_partition_table_type(
    ) -> crate::Result<()> {
        let bytes: Vec<u8> = b"atari".to_vec();
        let actual = PartitionTableType::try_from(bytes)?;
        let expected = PartitionTableType::Atari;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn partition_table_type_can_parse_a_valid_type() -> crate::Result<()> {
        let type_str = "aix";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::AIX;
        assert_eq!(actual, expected);

        let type_str = "atari";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Atari;
        assert_eq!(actual, expected);

        let type_str = "bsd";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::BSD;
        assert_eq!(actual, expected);

        let type_str = "dos";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::DOS;
        assert_eq!(actual, expected);

        let type_str = "freebsd";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::FreeBSD;
        assert_eq!(actual, expected);

        let type_str = "gpt";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::GPT;
        assert_eq!(actual, expected);

        let type_str = "mac";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Mac;
        assert_eq!(actual, expected);

        let type_str = "minix";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Minix;
        assert_eq!(actual, expected);

        let type_str = "netbsd";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::NetBSD;
        assert_eq!(actual, expected);

        let type_str = "openbsd";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::OpenBSD;
        assert_eq!(actual, expected);

        let type_str = "PMBR";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::ProtectiveMBR;
        assert_eq!(actual, expected);

        let type_str = "sgi";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::SGI;
        assert_eq!(actual, expected);

        let type_str = "solaris";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::SolarisX86;
        assert_eq!(actual, expected);

        let type_str = "sun";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Sun;
        assert_eq!(actual, expected);

        let type_str = "ultrix";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Ultrix;
        assert_eq!(actual, expected);

        let type_str = "unixware";
        let actual: PartitionTableType = type_str.parse()?;
        let expected = PartitionTableType::Unixware;
        assert_eq!(actual, expected);

        Ok(())
    }
}
