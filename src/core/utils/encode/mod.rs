// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Encode strings to a safe, udev-compatible format.

// From dependency library

// From standard library

// From this library
use crate::core::errors::EncodeError;

use crate::ffi_utils;

/// Encodes all potentially unsafe characters of a string to the corresponding hex value prefixed by `\x`.
pub fn encode_string<T>(string: T) -> Result<String, EncodeError>
where
    T: AsRef<[u8]>,
{
    let mut string = string.as_ref().to_vec();
    // Add '\0' C string terminator.
    string.push(0);

    log::debug!("encode::encode_string encoding {:?}", string);

    let max_length = string.len() * 8;
    let mut encoded: Vec<libc::c_char> = vec![0; max_length];

    let result = unsafe {
        libblkid::blkid_encode_string(
            string.as_ptr() as *const _,
            encoded.as_mut_ptr(),
            max_length,
        )
    };

    match result {
        0 => {
            let encoded_string = ffi_utils::c_char_array_to_string(encoded.as_ptr());
            log::debug!(
                "encode::encode_string encoded {:?} to {:?}",
                string,
                encoded_string
            );

            Ok(encoded_string)
        }
        code => {
            let err_msg = format!("failed to encode unsafe characters in string: {:?}", string);
            log::debug!(
                "encode::encode_string {}. libblkid::blkid_encode_string returned error code: {:?}",
                err_msg,
                code
            );

            Err(EncodeError::StringEncoding(err_msg))
        }
    }
}

/// Processes white-space characters. Keeps all valid ASCII and UTF-8 characters, then replaces everything else with `_`.
pub fn to_safe_string<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    let mut bytes = bytes.as_ref().to_vec();
    // Add '\0' C string terminator.
    bytes.push(0);

    log::debug!(
        "encode::to_safe_string converting bytes {:?} to safe string",
        bytes
    );

    let max_length = bytes.len() * 4;
    if max_length == 0 {
        "".to_string()
    } else {
        let mut encoded: Vec<libc::c_char> = vec![0; max_length];

        // `blkid_safe_string` returns an error only if we provide NULL pointer arguments or a
        // zero max_length. Since this will never be the case, we can safely ignore its return code.
        let _result = unsafe {
            libblkid::blkid_safe_string(
                bytes.as_ptr() as *const _,
                encoded.as_mut_ptr(),
                max_length,
            )
        };

        let safe_string = ffi_utils::c_char_array_to_string(encoded.as_ptr());
        log::debug!(
            "encode::to_safe_string converted bytes {:?} to a safe string: {:?}",
            bytes,
            safe_string
        );

        safe_string
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::ffi::CString;

    #[test]
    fn to_safe_string_correctly_processes_an_empty_byte_string() {
        let bytes = b"".to_vec();
        let actual = to_safe_string(bytes);
        let expected = String::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn to_safe_string_correctly_processes_whitespace() {
        let bytes = b"text with white space".to_vec();
        let actual = to_safe_string(bytes);
        let expected = String::from("text_with_white_space");
        assert_eq!(actual, expected);
    }

    #[test]
    fn to_safe_string_correctly_processes_non_utf8_characters() {
        let bytes: Vec<u8> = vec![
            116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 110, 111, 110, 45, 117, 116, 102, 56,
            0xBA, 0xDD,
        ];

        let actual = to_safe_string(bytes);
        let expected = String::from("text_with_non-utf8__");
        assert_eq!(actual, expected);
    }
}
