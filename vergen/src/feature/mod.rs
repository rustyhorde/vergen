// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "build")]
pub(crate) mod build;
#[cfg(feature = "cargo")]
pub(crate) mod cargo;
#[cfg(feature = "rustc")]
pub(crate) mod rustc;
#[cfg(feature = "si")]
pub(crate) mod si;
