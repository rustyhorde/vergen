// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(all(test, feature = "cargo"))]
pub(crate) mod testutils {
    use std::env;

    pub(crate) fn setup() {
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
        env::set_var("PROFILE", "debug");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("CARGO_FEATURE_BUILD", "build");
    }

    pub(crate) fn teardown() {
        env::remove_var("TARGET");
        env::remove_var("PROFILE");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("CARGO_FEATURE_BUILD");
    }
}
