extern crate vergen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use vergen::*;

#[test]
fn test_vergen() {
    let tmp = env::temp_dir();
    env::set_var("OUT_DIR",&tmp);
    env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    let mut flags = Flags::all();
    flags.toggle(NOW);
    vergen(flags);
    match File::open(&tmp.join("version.rs")) {
        Ok(ref mut f) => {
            match f.metadata() {
                Ok(meta) => assert!(meta.is_file()),
                Err(_)   => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
