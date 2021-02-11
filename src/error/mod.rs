// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` errors

mod codes;
mod sources;

pub(crate) use codes::ErrCode;
pub(crate) use sources::ErrSource;

use getset::Getters;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// A result that must include an `crate::error::Error`
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An error from the library
#[derive(Debug, Deserialize, Getters, Serialize)]
#[get = "pub(crate)"]
pub struct Error {
    /// the code
    code: ErrCode,
    /// the reason
    reason: String,
    /// the source
    #[serde(skip)]
    source: Option<ErrSource>,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.reason == other.reason
            && ((self.source.is_some() && other.source.is_some())
                || (self.source.is_none() && other.source.is_none()))
    }
}

impl Error {
    /// Create a new error
    pub(crate) fn new<U>(code: ErrCode, reason: U, source: Option<ErrSource>) -> Self
    where
        U: Into<String>,
    {
        let reason = reason.into();

        Self {
            code,
            reason,
            source,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|x| {
            let y: &(dyn std::error::Error + 'static) = x;
            y
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.reason)
        // let err: &(dyn std::error::Error) = self;
        // let mut iter = err.chain();
        // let _skip_me = iter.next();
        // for e in iter {
        //     writeln!(f)?;
        //     write!(f, "{}", e)?;
        // }
        // Ok(())
    }
}

impl From<&str> for Error {
    fn from(text: &str) -> Self {
        let split = text.split(':');
        let vec = split.collect::<Vec<&str>>();
        let code = vec.get(0).unwrap_or(&"");
        let reason = vec.get(1).unwrap_or(&"");
        Self::new((*code).into(), *reason, None)
    }
}

impl From<String> for Error {
    fn from(text: String) -> Self {
        let split = text.split(':');
        let vec = split.collect::<Vec<&str>>();
        let code = vec.get(0).unwrap_or(&"");
        let reason = vec.get(1).unwrap_or(&"");
        Self::new((*code).into(), *reason, None)
    }
}

#[cfg(test)]
mod test {
    use super::{ErrCode, Error};

    #[test]
    fn from_string_works() {
        assert_eq!(
            Error::from("protocol:err".to_string()),
            Error::new("protocol".into(), "err", None)
        )
    }

    #[test]
    fn from_str_works() {
        assert_eq!(
            Error::from("protocol:err"),
            Error::new("protocol".into(), "err", None)
        )
    }

    #[test]
    fn error_source() {
        assert!(Error::new(ErrCode::Protocol, "err", None)
            .source()
            .is_none());
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Error::new(ErrCode::Protocol, "err", None)),
            "protocol: err"
        );
    }
}
