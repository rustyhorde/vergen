// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Defines the `vergen` tests.
extern crate regex;
extern crate vergen;

use regex::Regex;
use std::env;
use std::fs::File;
use vergen::{vergen, ConstantsFlags, Vergen, VergenKey};

#[test]
fn test_vergen() {
    let tmp = env::temp_dir();
    env::set_var("OUT_DIR", &tmp);
    env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
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

#[test]
fn test_build_info_all() {
    let flags = ConstantsFlags::all();
    let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
    let build_info = vergen.build_info();

    assert!(build_info.contains_key(&VergenKey::BuildTimestamp));
    assert!(build_info.contains_key(&VergenKey::BuildDate));
    assert!(build_info.contains_key(&VergenKey::Sha));
    assert!(build_info.contains_key(&VergenKey::ShortSha));
    assert!(build_info.contains_key(&VergenKey::CommitDate));
    assert!(build_info.contains_key(&VergenKey::TargetTriple));
    assert!(build_info.contains_key(&VergenKey::Semver));
    assert!(build_info.contains_key(&VergenKey::SemverLightweight));
}

#[test]
fn test_build_info_some() {
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::COMMIT_DATE);
    flags.toggle(ConstantsFlags::SEMVER_LIGHTWEIGHT);
    let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
    let build_info = vergen.build_info();

    assert!(build_info.contains_key(&VergenKey::BuildTimestamp));
    assert!(build_info.contains_key(&VergenKey::BuildDate));
    assert!(build_info.contains_key(&VergenKey::Sha));
    assert!(build_info.contains_key(&VergenKey::ShortSha));
    assert!(!build_info.contains_key(&VergenKey::CommitDate));
    assert!(build_info.contains_key(&VergenKey::TargetTriple));
    assert!(build_info.contains_key(&VergenKey::Semver));
    assert!(!build_info.contains_key(&VergenKey::SemverLightweight));
}

#[test]
fn test_build_info_none() {
    let flags = ConstantsFlags::empty();
    let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
    let build_info = vergen.build_info();

    assert!(!build_info.contains_key(&VergenKey::BuildTimestamp));
    assert!(!build_info.contains_key(&VergenKey::BuildDate));
    assert!(!build_info.contains_key(&VergenKey::Sha));
    assert!(!build_info.contains_key(&VergenKey::ShortSha));
    assert!(!build_info.contains_key(&VergenKey::CommitDate));
    assert!(!build_info.contains_key(&VergenKey::TargetTriple));
    assert!(!build_info.contains_key(&VergenKey::Semver));
    assert!(!build_info.contains_key(&VergenKey::SemverLightweight));
}

#[test]
fn test_build_info_commit_date() {
    let mut flags = ConstantsFlags::empty();
    flags.toggle(ConstantsFlags::COMMIT_DATE);
    let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
    let build_info = vergen.build_info();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Unable to create date regex!");
    if let Some(commit_date) = build_info.get(&VergenKey::CommitDate) {
        assert!(re.is_match(commit_date));
    } else {
        assert!(false, "The commit date wasn't set properly");
    }
}
