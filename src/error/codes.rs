// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `att_api` error codes

use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// Error Codes
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub(crate) enum ErrCode {
    /// An environmental error
    Env,
    /// An I/O error
    Io,
    /// A parsing error
    Parse,
    /// A protocol error
    Protocol,
    /// An unknown
    Unknown,
}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Env => "env",
                Self::Io => "io",
                Self::Parse => "parse",
                Self::Protocol => "protocol",
                Self::Unknown => "unknown",
            }
        )
    }
}

impl Into<&str> for ErrCode {
    #[must_use]
    fn into(self) -> &'static str {
        match self {
            Self::Env => "env",
            Self::Io => "io",
            Self::Parse => "parse",
            Self::Protocol => "protocol",
            Self::Unknown => "unknown",
        }
    }
}

impl Into<String> for ErrCode {
    #[must_use]
    fn into(self) -> String {
        let tmp: &str = self.into();
        tmp.to_string()
    }
}

impl From<&str> for ErrCode {
    #[must_use]
    fn from(text: &str) -> Self {
        match text {
            "env" => Self::Env,
            "io" => Self::Io,
            "parse" => Self::Parse,
            "protocol" => Self::Protocol,
            _ => Self::Unknown,
        }
    }
}
