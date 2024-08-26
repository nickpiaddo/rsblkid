// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fmt;
use std::str::FromStr;

// From this library
use crate::core::errors::ParserError;
use crate::core::utils::encode;

/// Raw bytes.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RawBytes {
    bytes: Vec<u8>,
    byte_str_lossy: String,
    byte_str_safe: String,
}

impl RawBytes {
    /// View this `RawBytes` as a lossy UTF-8 `str`. `as_str_lossy()` will replace any invalid
    /// UTF-8 sequences with [U+FFFD REPLACEMENT
    /// CHARACTER](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html), which
    /// looks like this: �
    pub fn as_str_lossy(&self) -> &str {
        &self.byte_str_lossy
    }

    /// View this `RawBytes` as a safe string where ASCII, and UTF-8 characters are preserved while
    /// non-printable characters are replaced with underscores.
    pub fn as_str_safe(&self) -> &str {
        &self.byte_str_safe
    }

    /// Returns a byte slice of this `RawBytes`'s content.
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl AsRef<RawBytes> for RawBytes {
    #[inline]
    fn as_ref(&self) -> &RawBytes {
        self
    }
}

impl<T> From<T> for RawBytes
where
    T: AsRef<[u8]>,
{
    fn from(bytes: T) -> RawBytes {
        let bytes = bytes.as_ref().to_owned();
        let byte_str_lossy = String::from_utf8_lossy(&bytes).to_string();
        let byte_str_safe = encode::to_safe_string(&bytes);

        Self {
            bytes,
            byte_str_lossy,
            byte_str_safe,
        }
    }
}

impl fmt::Display for RawBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.byte_str_safe)
    }
}

impl FromStr for RawBytes {
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
                .ok_or(ParserError::RawBytes(err_missing_dquote))
        } else if trimmed.starts_with('\'') {
            trimmed
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .ok_or(ParserError::RawBytes(err_missing_quote))
        } else {
            Ok(trimmed)
        }?;

        let bytes = stripped.as_bytes().to_vec();
        let byte_str_lossy = stripped.to_string();
        let byte_str_safe = encode::to_safe_string(&bytes);

        let raw_bytes = Self {
            bytes,
            byte_str_lossy,
            byte_str_safe,
        };

        Ok(raw_bytes)
    }
}
