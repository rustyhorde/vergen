// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Defines the `vergen` tests.
extern crate vergen;

use std::env;
use std::fs::File;
use vergen::{vergen, ConstantsFlags, COMPILE_TIME};

#[test]
fn test_vergen() {
    let tmp = env::temp_dir();
    env::set_var("OUT_DIR", &tmp);
    env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    let mut flags = ConstantsFlags::all();
    flags.toggle(COMPILE_TIME);
    match vergen(flags) {
        Ok(_) => assert!(true),
        Err(e) => assert!(false, format!("{}", e)),
    }
    match File::open(&tmp.join("version.rs")) {
        Ok(ref mut f) => match f.metadata() {
            Ok(meta) => assert!(meta.is_file()),
            Err(_) => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
