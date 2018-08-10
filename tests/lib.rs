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
fn test_build_info_data() {
    let flags = ConstantsFlags::all();
    let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
    let build_info = vergen.build_info();

    let timestamp_re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.(\d+)\+\d{2}:\d{2}$")
        .expect("Unable to create timestamp regex!");
    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Unable to create date regex!");
    let sha_re = Regex::new(r"^[a-z0-9]{40}$").expect("Unable to create SHA regex!");
    let short_sha_re = Regex::new(r"^[a-z0-9]{7}").expect("Unable to create short SHA regex!");
    let target_triple_re = Regex::new(r"^[a-z0-9_]+-[a-z0-9_]+-[a-z0-9_]+-[a-z0-9_]+$")
        .expect("Unable to create target triple regex!");
    let semver_re =
        Regex::new(r"^v*\d+\.\d+\.\d+([-a-z.0-9]+)?$").expect("Unable to create semver regex!");

    if let Some(build_timestamp) = build_info.get(&VergenKey::BuildTimestamp) {
        assert!(timestamp_re.is_match(build_timestamp));
    } else {
        assert!(false, "The build timestamp wasn't properly set");
    }

    if let Some(build_date) = build_info.get(&VergenKey::BuildDate) {
        assert!(date_re.is_match(build_date));
    } else {
        assert!(false, "The build date wasn't set properly");
    }

    if let Some(sha) = build_info.get(&VergenKey::Sha) {
        assert!(sha_re.is_match(sha));
    } else {
        assert!(false, "The SHA wasn't set properly");
    }

    if let Some(short_sha) = build_info.get(&VergenKey::Sha) {
        assert!(short_sha_re.is_match(short_sha));
    } else {
        assert!(false, "The short SHA wasn't set properly");
    }

    if let Some(commit_date) = build_info.get(&VergenKey::CommitDate) {
        assert!(date_re.is_match(commit_date));
    } else {
        assert!(false, "The commit date wasn't set properly");
    }

    if let Some(target_triple) = build_info.get(&VergenKey::TargetTriple) {
        assert!(target_triple_re.is_match(target_triple));
    } else {
        assert!(false, "The commit date wasn't set properly");
    }

    if let Some(semver) = build_info.get(&VergenKey::Semver) {
        assert!(semver_re.is_match(semver));
    } else {
        assert!(false, "The semver wasn't set properly");
    }

    if let Some(semver_lightweight) = build_info.get(&VergenKey::SemverLightweight) {
        assert!(semver_re.is_match(semver_lightweight));
    } else {
        assert!(false, "The lightweight semver wasn't set properly");
    }
}
