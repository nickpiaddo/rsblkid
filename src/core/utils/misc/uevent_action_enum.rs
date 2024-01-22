// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::ffi::CString;
use std::fmt;

// From this library

/// Types of `uevent` actions.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum UEventAction {
    Add,
    Change,
    Remove,
}

impl UEventAction {
    pub fn as_str(&self) -> &str {
        match self {
            UEventAction::Add => "add",
            UEventAction::Change => "change",
            UEventAction::Remove => "remove",
        }
    }
    pub fn to_c_string(&self) -> CString {
        CString::new(self.as_str()).unwrap()
    }
}

impl fmt::Display for UEventAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
