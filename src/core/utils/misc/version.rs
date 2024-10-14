// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::mem::MaybeUninit;

// From this library
use crate::core::utils::misc::{LibraryInfo, MiscError};
use crate::ffi_utils;

/// Returns general information about the `libblkid` library.
pub fn library_version() -> LibraryInfo {
    let mut version_string_ptr = MaybeUninit::<*const libc::c_char>::zeroed();
    let mut release_date_ptr = MaybeUninit::<*const libc::c_char>::zeroed();

    let release_code = unsafe {
        libblkid::blkid_get_library_version(
            version_string_ptr.as_mut_ptr(),
            release_date_ptr.as_mut_ptr(),
        )
    };

    let version_ptr = unsafe { version_string_ptr.assume_init() };
    let date_ptr = unsafe { release_date_ptr.assume_init() };

    let version_string = ffi_utils::c_char_array_to_string(version_ptr);
    let release_date = ffi_utils::c_char_array_to_string(date_ptr);

    let info = LibraryInfo::new(release_code, release_date, version_string);
    log::debug!("misc::library_version library version: {:?}", info);

    info
}

/// Converts a version string to the corresponding release code.
///
/// # Examples
///
/// ```
/// # use pretty_assertions::assert_eq;
/// use rsblkid::core::utils::misc;
///
/// fn main() -> rsblkid::Result<()> {
///     let version_string = "2.38.1";
///     let actual = misc::version_string_to_release_code(version_string)?;
///     let expected = 2381;
///     assert_eq!(actual, expected);
///
///     Ok(())
/// }
/// ```
pub fn version_string_to_release_code<T>(version_string: T) -> Result<u32, MiscError>
where
    T: AsRef<str>,
{
    let version_string = version_string.as_ref();
    let version_cstr = ffi_utils::as_ref_str_to_c_string(version_string)?;

    let version_code = unsafe { libblkid::blkid_parse_version_string(version_cstr.as_ptr()) };
    log::debug!(
        "misc::version_string_to_release_code converted version string {:?} to release code {:?}",
        version_string,
        version_code
    );

    Ok(version_code as u32)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn version_string_to_release_code_converts_valid_version_string() {
        let version_string = "2.38.1";
        let expected = 2381;
        let result = version_string_to_release_code(version_string).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn version_string_to_release_code_fails_on_invalid_c_string() {
        let version_string = String::from_utf8(b"2.38\0.1".to_vec()).unwrap();
        let _result = version_string_to_release_code(version_string).unwrap();
    }

    #[test]
    fn version_string_to_release_code_converts_up_to_first_invalid_character() {
        let version_string = "v2.38.1";
        let expected = 0;
        let result = version_string_to_release_code(version_string).unwrap();

        assert_eq!(result, expected);

        let version_string = "2.38.x";
        let expected = 238;
        let result = version_string_to_release_code(version_string).unwrap();

        assert_eq!(result, expected);
    }
}
