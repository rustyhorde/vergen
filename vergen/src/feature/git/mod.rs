// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "gitcl")]
pub(crate) mod cmd;
#[cfg(feature = "git2")]
pub(crate) mod git2;
#[cfg(feature = "gix")]
pub(crate) mod gix;

#[cfg(all(feature = "git", feature = "gitcl"))]
pub(crate) use self::cmd::Config;
#[cfg(all(feature = "git", feature = "git2"))]
pub(crate) use self::git2::Config;
#[cfg(all(feature = "git", feature = "gix"))]
pub(crate) use self::gix::Config;
