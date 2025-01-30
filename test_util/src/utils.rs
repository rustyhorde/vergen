use std::sync::LazyLock;

use anyhow::Result;
use temp_env::with_vars;

static DEFAULT_KVS: LazyLock<Vec<(&'static str, Option<&'static str>)>> = LazyLock::new(|| {
    vec![
        ("CARGO_FEATURE_BUILD", Some("build")),
        ("CARGO_FEATURE_GIT", Some("git")),
        ("DEBUG", Some("true")),
        ("OPT_LEVEL", Some("1")),
        ("TARGET", Some("x86_64-unknown-linux-gnu")),
    ]
});

/// Wrap a closure with cargo environment variables to use within a test
///
/// * `CARGO_FEATURE_BUILD=build`
/// * `CARGO_FEATURE_GIT=git`
/// * `DEBUG=true`
/// * `OPT_LEVEL=1`
/// * `TARGET=x86_64-unknown-linux-gnu`
///
/// Uses [`temp_env::with_vars`] internally to provide a safe environment to do this with tests.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use std::env::var;
/// # use test_util::with_cargo_vars;
/// #
/// let result = with_cargo_vars(|| {
///     assert_eq!(var("OPT_LEVEL")?, "1");
///     Ok(())
/// });
/// assert!(result.is_ok());
/// ```
///
/// # Errors
///
/// Errors may be generate by the closure
///
pub fn with_cargo_vars<F, R>(closure: F) -> Result<R>
where
    F: FnOnce() -> Result<R>,
{
    with_cargo_vars_ext(&[], closure)
}

/// Wrap a closure with cargo environment variables to use within a test
/// plus any other environment variables you may need.
///
/// * `CARGO_FEATURE_BUILD=build`
/// * `CARGO_FEATURE_GIT=git`
/// * `DEBUG=true`
/// * `OPT_LEVEL=1`
/// * `TARGET=x86_64-unknown-linux-gnu`
/// * `MY_FUNKY_ENV=this`
///
/// Uses [`temp_env::with_vars`] internally to provide a safe environment to do this with tests.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use std::env::var;
/// # use test_util::with_cargo_vars_ext;
/// #
/// let result = with_cargo_vars_ext(&[("MY_VAR", Some("12"))], || {
///     assert_eq!(var("MY_VAR")?, "12");
///     assert_eq!(var("OPT_LEVEL")?, "1");
///     Ok(())
/// });
/// assert!(result.is_ok());
/// ```
///
/// # Errors
///
/// Errors may be generate by the closure
///
pub fn with_cargo_vars_ext<F, R>(kvs: &[(&str, Option<&str>)], closure: F) -> Result<R>
where
    F: FnOnce() -> Result<R>,
{
    let mut in_kvs: Vec<(&str, Option<&str>)> = kvs.as_ref().to_vec();
    in_kvs.extend_from_slice(&DEFAULT_KVS);
    with_vars(in_kvs, closure)
}
