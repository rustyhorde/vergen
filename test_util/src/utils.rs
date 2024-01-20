use temp_env::with_vars;

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
    let mut default_kvs = [
        ("CARGO_FEATURE_BUILD", Some("build")),
        ("CARGO_FEATURE_GIT", Some("git")),
        ("DEBUG", Some("true")),
        ("OPT_LEVEL", Some("1")),
        ("TARGET", Some("x86_64-unknown-linux-gnu")),
    ]
    .to_vec();
    in_kvs.append(&mut default_kvs);
    with_vars(in_kvs, closure);
}
