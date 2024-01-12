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
        env::set_var("CARGO_FEATURE_BUILD", "build");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("DEBUG", "true");
        env::set_var("OPT_LEVEL", "1");
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    }

    pub(crate) fn teardown() {
        env::remove_var("CARGO_FEATURE_BUILD");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("DEBUG");
        env::remove_var("OPT_LEVEL");
        env::remove_var("TARGET");
    }
}

#[cfg(any(
    feature = "build",
    feature = "cargo",
    all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ),
    feature = "rustc",
    feature = "si",
))]
pub(crate) mod fns {
    use crate::{constants::VERGEN_IDEMPOTENT_DEFAULT, emitter::RustcEnvMap, key::VergenKey};
    use std::env;

    pub(crate) fn add_default_map_entry(
        key: VergenKey,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if let Ok(value) = env::var(key.name()) {
            add_map_entry(key, value, map);
            warnings.push(format!("{} overidden", key.name()));
        } else {
            add_map_entry(key, VERGEN_IDEMPOTENT_DEFAULT, map);
            warnings.push(format!("{} set to default", key.name()));
        }
    }

    pub(crate) fn add_map_entry<T>(key: VergenKey, value: T, map: &mut RustcEnvMap)
    where
        T: Into<String>,
    {
        let _old = map.insert(key, value.into());
    }
}
