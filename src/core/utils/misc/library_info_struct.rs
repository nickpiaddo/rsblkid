// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library

/// `rsblkid` version information.
#[derive(Debug)]
pub struct LibraryInfo {
    release_code: i32,
    release_date: String,
    version_string: String,
}

impl LibraryInfo {
    pub(crate) fn new(
        release_code: i32,
        release_date: String,
        version_string: String,
    ) -> LibraryInfo {
        Self {
            release_code,
            release_date,
            version_string,
        }
    }

    /// Returns the library's release code (e.g `2381`).
    pub fn release_code(&self) -> u32 {
        self.release_code as u32
    }

    /// Returns the library's release date (e.g. `04-Aug-2022`).
    pub fn release_date(&self) -> &str {
        &self.release_date
    }

    /// Returns the library's version string (e.g. `2.38.1`).
    pub fn version_string(&self) -> &str {
        &self.version_string
    }
}
