// Copyright (c) 2020 att-api developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `att_api` error sources

use crate::error::{ErrCode, Error};
use std::fmt;

macro_rules! dep_error {
    ($error:ty, $kind:expr, $code:expr, $reason:expr) => {
        impl From<$error> for Error {
            #[must_use]
            fn from(inner: $error) -> Self {
                Self::new($code, $reason, Some($kind(inner)))
            }
        }
    };
}

dep_error!(
    std::env::VarError,
    ErrSource::Var,
    ErrCode::Env,
    "There was an error processing your enviroment"
);
dep_error!(
    std::io::Error,
    ErrSource::Io,
    ErrCode::Io,
    "There was an error processing your request"
);
dep_error!(
    std::num::TryFromIntError,
    ErrSource::TryFromInt,
    ErrCode::Parse,
    "There was an error trying to convert an integer"
);
dep_error!(
    std::num::ParseIntError,
    ErrSource::ParseInt,
    ErrCode::Parse,
    "There was an error trying to convert to an integer"
);
dep_error!(
    std::array::TryFromSliceError,
    ErrSource::TryFromSlice,
    ErrCode::Protocol,
    "There was an error converting a slice to an array"
);
dep_error!(
    std::path::StripPrefixError,
    ErrSource::StripPrefix,
    ErrCode::Parse,
    "There was an error trying to strip a prefix from a path"
);
dep_error!((), ErrSource::Unit, ErrCode::Protocol, "There was an error");
#[cfg(feature = "git")]
dep_error!(
    git2::Error,
    ErrSource::Git2,
    ErrCode::Protocol,
    "There was an error from the git2 library"
);
#[cfg(feature = "rustc")]
dep_error!(
    rustc_version::Error,
    ErrSource::RustcVersion,
    ErrCode::Protocol,
    "There was an error from the rustc_version library"
);

/// Error Source
#[derive(Debug)]
#[allow(clippy::large_enum_variant, variant_size_differences)]
pub(crate) enum ErrSource {
    /// An error from the git2 library
    #[cfg(feature = "git")]
    Git2(git2::Error),
    /// An I/O error
    Io(std::io::Error),
    /// An error trying to convert to an integer type
    ParseInt(std::num::ParseIntError),
    /// An error from the rustc_version library
    #[cfg(feature = "rustc")]
    RustcVersion(rustc_version::Error),
    /// An error trying to strip a prefix from a path
    StripPrefix(std::path::StripPrefixError),
    /// An error trying to convert from an integer type
    TryFromInt(std::num::TryFromIntError),
    /// An error converting bytes to isize
    TryFromSlice(std::array::TryFromSliceError),
    /// A unit error
    Unit(()),
    /// An error reading an environment variable
    Var(std::env::VarError),
}

impl std::error::Error for ErrSource {}

impl fmt::Display for ErrSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "git")]
            Self::Git2(source) => write!(f, "{}", source),
            Self::Io(source) => write!(f, "{}", source),
            Self::ParseInt(source) => write!(f, "{}", source),
            #[cfg(feature = "rustc")]
            Self::RustcVersion(source) => write!(f, "{}", source),
            Self::StripPrefix(source) => write!(f, "{}", source),
            Self::TryFromInt(source) => write!(f, "{}", source),
            Self::TryFromSlice(source) => write!(f, "{}", source),
            Self::Unit(_source) => write!(f, "unit"),
            Self::Var(source) => write!(f, "{}", source),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Error;
    use crate::error::Result;
    #[cfg(feature = "git")]
    use git2::Repository;
    #[cfg(feature = "rustc")]
    use rustc_version::version_meta_for;
    use std::{
        convert::{TryFrom, TryInto},
        env,
        io::{self, ErrorKind},
        path::Path,
    };

    #[cfg(feature = "git")]
    #[test]
    fn git2_error() {
        let res = Repository::open("blah").map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!("protocol: There was an error from the git2 library - failed to resolve path \'blah\': No such file or directory; class=Os (2); code=NotFound (-3)", format!("{}", err));
    }

    #[test]
    fn io_error() {
        let err: Error = io::Error::new(ErrorKind::Other, "testing").into();
        assert_eq!(
            "io: There was an error processing your request - testing",
            format!("{}", err)
        );
    }

    #[test]
    fn parse_int_error() {
        let res = "a".parse::<u32>().map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(
            "parse: There was an error trying to convert to an integer - invalid digit found in string",
            format!("{}", err)
        );
    }

    #[cfg(feature = "rustc")]
    #[test]
    fn rustc_version_error() {
        let res = version_meta_for("yoda").map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!("protocol: There was an error from the rustc_version library - unexpected `rustc -vV` format", format!("{}", err));
    }

    #[test]
    fn strip_prefix_error() {
        let path = Path::new("/test/haha/foo.txt");
        let res = path.strip_prefix("test").map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(
            "parse: There was an error trying to strip a prefix from a path - prefix not found",
            format!("{}", err)
        );
    }

    #[test]
    fn try_from_int_error() {
        let res = u8::try_from(257).map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!("parse: There was an error trying to convert an integer - out of range integral type conversion attempted", format!("{}", err));
    }

    #[test]
    fn try_from_slice_error() {
        let res: Result<[u8; 3]> = "blah"
            .as_bytes()
            .iter()
            .copied()
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!("protocol: There was an error converting a slice to an array - could not convert slice to array", format!("{}", err));
    }

    #[test]
    fn unit_error() {
        let err: Error = ().into();
        assert_eq!("protocol: There was an error - unit", format!("{}", err));
    }

    #[test]
    fn var_error() {
        let res = env::var("yoda").map_err(|e| Error::from(e));
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(
            "env: There was an error processing your enviroment - environment variable not found",
            format!("{}", err)
        );
    }
}
