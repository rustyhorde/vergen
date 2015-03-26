#![feature(path_ext)]
extern crate vergen;

use std::env;
use std::io::prelude::*;
use vergen::vergen;

#[test]
fn test_vergen() {
    let tmp = env::temp_dir();
    env::set_var("OUT_DIR",&tmp);
    vergen();
    assert!(&tmp.join("version.rs").exists());
}
