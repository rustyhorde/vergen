#![feature(core,env,io,path)]
extern crate time;

use std::env;
use std::old_io::File;
use std::old_io::process::Command;

fn gen_now_fn() -> String {
    let mut now_fn = "pub fn now() -> &'static str {\n".to_string();

    let mut now = Command::new("date");
    now.arg("--rfc-3339=ns");

    match now.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(o.output.as_slice());
            now_fn.push_str("    \"");
            now_fn.push_str(po.trim());
            now_fn.push_str("\"\n");
            now_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    }

    now_fn
}

fn gen_sha_fn() -> String {
    let mut sha_fn = "pub fn sha() -> &'static str {\n".to_string();

    let mut sha_cmd = Command::new("git");
    sha_cmd.args(&["rev-parse", "HEAD"]);

    match sha_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(o.output.as_slice());
            sha_fn.push_str("    \"");
            sha_fn.push_str(po.trim());
            sha_fn.push_str("\"\n");
            sha_fn.push_str("}\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    sha_fn
}

fn gen_semver_fn() -> String {
    let mut semver_fn = "pub fn semver() -> &'static str {\n".to_string();

    let mut branch_cmd = Command::new("git");
    branch_cmd.args(&["describe"]);

    match branch_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(o.output.as_slice());
            semver_fn.push_str("    \"");
            semver_fn.push_str(po.trim());
            semver_fn.push_str("\"\n");
            semver_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    semver_fn
}

pub fn vergen() {
    let dst = Path::new(env::var_string("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("version.rs")).unwrap();
    f.write_str(gen_now_fn().as_slice()).unwrap();
    f.write_str(gen_sha_fn().as_slice()).unwrap();
    f.write_str(gen_semver_fn().as_slice()).unwrap();
}
