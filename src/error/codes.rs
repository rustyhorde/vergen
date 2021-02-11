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
#[derive(Copy, Clone, Debug, Eq, Deserialize, PartialEq, Serialize)]
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
        write!(f, "{}", to_str(*self))
    }
}

impl From<ErrCode> for &'static str {
    fn from(ec: ErrCode) -> &'static str {
        to_str(ec)
    }
}

fn to_str(ec: ErrCode) -> &'static str {
    match ec {
        ErrCode::Env => "env",
        ErrCode::Io => "io",
        ErrCode::Parse => "parse",
        ErrCode::Protocol => "protocol",
        ErrCode::Unknown => "unknown",
    }
}

impl From<ErrCode> for String {
    fn from(ec: ErrCode) -> String {
        format!("{}", ec)
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

#[cfg(test)]
mod test {
    use super::ErrCode;
    use crate::error::Result;
    use std::io::Write;

    #[test]
    fn display_works() -> Result<()> {
        let mut buf = vec![];
        write!(buf, "{}", ErrCode::Env)?;
        assert_eq!("env", String::from_utf8_lossy(&buf));
        buf.clear();
        write!(buf, "{}", ErrCode::Io)?;
        assert_eq!("io", String::from_utf8_lossy(&buf));
        buf.clear();
        write!(buf, "{}", ErrCode::Parse)?;
        assert_eq!("parse", String::from_utf8_lossy(&buf));
        buf.clear();
        write!(buf, "{}", ErrCode::Protocol)?;
        assert_eq!("protocol", String::from_utf8_lossy(&buf));
        buf.clear();
        write!(buf, "{}", ErrCode::Unknown)?;
        assert_eq!("unknown", String::from_utf8_lossy(&buf));
        buf.clear();
        Ok(())
    }

    #[test]
    fn from_str_works() {
        assert_eq!(ErrCode::Env, "env".into());
        assert_eq!(ErrCode::Io, "io".into());
        assert_eq!(ErrCode::Parse, "parse".into());
        assert_eq!(ErrCode::Protocol, "protocol".into());
        assert_eq!(ErrCode::Unknown, "unknown".into());
    }

    #[test]
    fn from_err_code_to_str_works() {
        assert_eq!(<&str>::from(ErrCode::Env), "env");
        assert_eq!(<&str>::from(ErrCode::Io), "io");
        assert_eq!(<&str>::from(ErrCode::Parse), "parse");
        assert_eq!(<&str>::from(ErrCode::Protocol), "protocol");
        assert_eq!(<&str>::from(ErrCode::Unknown), "unknown");
    }

    #[test]
    fn from_err_code_to_string_works() {
        assert_eq!(String::from(ErrCode::Env), "env".to_string());
        assert_eq!(String::from(ErrCode::Io), "io".to_string());
        assert_eq!(String::from(ErrCode::Parse), "parse".to_string());
        assert_eq!(String::from(ErrCode::Protocol), "protocol".to_string());
        assert_eq!(String::from(ErrCode::Unknown), "unknown".to_string());
    }
}
