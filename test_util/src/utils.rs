use lazy_static::lazy_static;
use temp_env::with_vars;

lazy_static! {
    static ref DEFAULT_KVS: Vec<(&'static str, Option<&'static str>)> = vec![
        ("CARGO_FEATURE_BUILD", Some("build")),
        ("CARGO_FEATURE_GIT", Some("git")),
        ("DEBUG", Some("true")),
        ("OPT_LEVEL", Some("1")),
        ("TARGET", Some("x86_64-unknown-linux-gnu")),
    ];
}

///
pub fn with_cargo_vars<F>(closure: F)
where
    F: FnOnce(),
{
    with_cargo_vars_ext(&[], closure);
}

///
pub fn with_cargo_vars_ext<F>(kvs: &[(&str, Option<&str>)], closure: F)
where
    F: FnOnce(),
{
    let mut in_kvs: Vec<(&str, Option<&str>)> = kvs.as_ref().to_vec();
    in_kvs.extend_from_slice(&DEFAULT_KVS);
    with_vars(in_kvs, closure);
}
