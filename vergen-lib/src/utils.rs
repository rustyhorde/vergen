use crate::{
    CargoRustcEnvMap, CargoWarning, VergenKey,
    constants::{VERGEN_IDEMPOTENT_DEFAULT, VERGEN_UNAVAILABLE_DEFAULT},
};
use std::env;

/// Add a [`VergenKey`] entry as a default string into the [`CargoRustcEnvMap`].
/// The value is either from an environment variable override or [`crate::constants::VERGEN_IDEMPOTENT_DEFAULT`]
///
/// # Example
/// ```
/// # use std::collections::BTreeMap;
/// # use temp_env::with_var;
/// # use vergen_lib::{add_default_map_entry, CargoRustcEnvMap, CargoWarning, VergenKey};
/// with_var("VERGEN_BUILD_DATE", Some("my own date"), || {
///     let mut map: CargoRustcEnvMap = BTreeMap::new();
///     let mut warning: CargoWarning = vec![];
#[cfg_attr(
    feature = "build",
    doc = r"    add_default_map_entry(false, VergenKey::BuildDate, &mut map, &mut warning);
assert_eq!(1, map.len());
assert_eq!(1, warning.len());"
)]
/// });
/// ```
///
pub fn add_default_map_entry(
    idempotent: bool,
    key: VergenKey,
    map: &mut CargoRustcEnvMap,
    warnings: &mut CargoWarning,
) {
    if let Ok(value) = env::var(key.name()) {
        add_map_entry(key, value, map);
        warnings.push(format!("{} overidden", key.name()));
    } else if idempotent {
        add_map_entry(key, VERGEN_IDEMPOTENT_DEFAULT, map);
        warnings.push(format!("{} set to default", key.name()));
    } else {
        warnings.push(format!("Unable to set {}", key.name()));
    }
}

/// Add a [`VergenKey`] entry for a value that could not be generated.
///
/// The value is either from an environment variable override or, when `add_placeholder`
/// is set, [`crate::constants::VERGEN_UNAVAILABLE_DEFAULT`]. Otherwise the entry is
/// left unset and a warning is recorded.
///
/// Use this on failure paths (for example in
/// [`AddEntries::add_default_entries`](crate::AddEntries::add_default_entries)) and
/// [`add_default_map_entry`] for values deliberately replaced in idempotent mode.
///
/// # Example
/// ```
/// # use std::collections::BTreeMap;
/// # use vergen_lib::{add_unavailable_map_entry, CargoRustcEnvMap, CargoWarning, VergenKey};
/// let mut map: CargoRustcEnvMap = BTreeMap::new();
/// let mut warning: CargoWarning = vec![];
#[cfg_attr(
    feature = "build",
    doc = r#"add_unavailable_map_entry(true, VergenKey::BuildDate, &mut map, &mut warning);
assert_eq!(Some(&"VERGEN_UNAVAILABLE".to_string()), map.get(&VergenKey::BuildDate));
assert_eq!(1, warning.len());"#
)]
/// ```
///
pub fn add_unavailable_map_entry(
    add_placeholder: bool,
    key: VergenKey,
    map: &mut CargoRustcEnvMap,
    warnings: &mut CargoWarning,
) {
    if let Ok(value) = env::var(key.name()) {
        add_map_entry(key, value, map);
        warnings.push(format!("{} overidden", key.name()));
    } else if add_placeholder {
        add_map_entry(key, VERGEN_UNAVAILABLE_DEFAULT, map);
        warnings.push(format!("{} set to default", key.name()));
    } else {
        warnings.push(format!("Unable to set {}", key.name()));
    }
}

/// Add a [`VergenKey`] entry as a string into the [`CargoRustcEnvMap`].
///
/// # Example
/// ```
/// # use std::collections::BTreeMap;
/// # use vergen_lib::{add_map_entry, CargoRustcEnvMap, VergenKey};
/// let mut map: CargoRustcEnvMap = BTreeMap::new();
#[cfg_attr(
    feature = "build",
    doc = r#"add_map_entry(VergenKey::BuildDate, "test", &mut map);
assert_eq!(1, map.len());"#
)]
/// ```
///
pub fn add_map_entry<T>(key: VergenKey, value: T, map: &mut CargoRustcEnvMap)
where
    T: Into<String>,
{
    let _old = map.insert(key, value.into());
}

/// Count the number of idempotent entries in a [`CargoRustcEnvMap`]
///
/// **NOTE** - This is mainly used for testing.
///
/// # Example
///
/// ```
/// # use std::collections::BTreeMap;
/// # use vergen_lib::{count_idempotent, CargoRustcEnvMap, VergenKey, constants::VERGEN_IDEMPOTENT_DEFAULT};
/// #
/// let mut map: CargoRustcEnvMap = BTreeMap::new();
/// assert_eq!(0, count_idempotent(&map));
#[cfg_attr(
    feature = "build",
    doc = r"_ = map.insert(VergenKey::BuildDate, VERGEN_IDEMPOTENT_DEFAULT.to_string());"
)]
#[cfg_attr(feature = "build", doc = r"assert_eq!(1, count_idempotent(&map));")]
/// ```
///
#[must_use]
pub fn count_idempotent(map: &CargoRustcEnvMap) -> usize {
    map.values()
        .filter(|x| *x == VERGEN_IDEMPOTENT_DEFAULT)
        .count()
}

/// Count the number of unavailable entries in a [`CargoRustcEnvMap`]
///
/// **NOTE** - This is mainly used for testing.
///
/// # Example
///
/// ```
/// # use std::collections::BTreeMap;
/// # use vergen_lib::{count_unavailable, CargoRustcEnvMap, VergenKey, constants::VERGEN_UNAVAILABLE_DEFAULT};
/// #
/// let mut map: CargoRustcEnvMap = BTreeMap::new();
/// assert_eq!(0, count_unavailable(&map));
#[cfg_attr(
    feature = "build",
    doc = r"_ = map.insert(VergenKey::BuildDate, VERGEN_UNAVAILABLE_DEFAULT.to_string());"
)]
#[cfg_attr(feature = "build", doc = r"assert_eq!(1, count_unavailable(&map));")]
/// ```
///
#[must_use]
pub fn count_unavailable(map: &CargoRustcEnvMap) -> usize {
    map.values()
        .filter(|x| *x == VERGEN_UNAVAILABLE_DEFAULT)
        .count()
}

/// Read the git SHA and dirty flag from a `.cargo_vcs_info.json` file at
/// `CARGO_MANIFEST_DIR`, if one is present.
///
/// This is the file [cargo includes in published crate tarballs][vcs-info], which
/// lets the git SHA and dirty flag be recovered when building from a package where
/// no `.git` directory exists (e.g. `cargo install` or a crates.io source).
///
/// Returns `Some((sha, dirty))` on success, or `None` if the file is absent or
/// cannot be parsed. `dirty` defaults to `false` when the field is missing.
///
/// [vcs-info]: https://doc.rust-lang.org/cargo/commands/cargo-package.html#cargo_vcs_infojson-format
#[cfg(feature = "vcs_info")]
#[must_use]
pub fn vcs_info() -> Option<(String, bool)> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")?;
    let path = std::path::Path::new(&manifest_dir).join(".cargo_vcs_info.json");
    let contents = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let git = json.get("git")?;
    let sha = git.get("sha1")?.as_str()?.to_string();
    let dirty = git
        .get("dirty")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    Some((sha, dirty))
}

#[cfg(all(test, feature = "build"))]
mod test {
    use super::{add_unavailable_map_entry, count_unavailable};
    use crate::{CargoRustcEnvMap, CargoWarning, VergenKey};
    use serial_test::serial;
    use std::collections::BTreeMap;
    use temp_env::with_var;

    #[test]
    #[serial]
    fn add_unavailable_map_entry_uses_env_override() {
        with_var("VERGEN_BUILD_DATE", Some("my own date"), || {
            let mut map: CargoRustcEnvMap = BTreeMap::new();
            let mut warnings: CargoWarning = vec![];
            add_unavailable_map_entry(true, VergenKey::BuildDate, &mut map, &mut warnings);
            assert_eq!(
                Some(&"my own date".to_string()),
                map.get(&VergenKey::BuildDate)
            );
            assert_eq!(0, count_unavailable(&map));
            assert_eq!(vec!["VERGEN_BUILD_DATE overidden".to_string()], warnings);
        });
    }

    #[test]
    #[serial]
    fn add_unavailable_map_entry_without_placeholder_warns() {
        with_var("VERGEN_BUILD_DATE", None::<&str>, || {
            let mut map: CargoRustcEnvMap = BTreeMap::new();
            let mut warnings: CargoWarning = vec![];
            add_unavailable_map_entry(false, VergenKey::BuildDate, &mut map, &mut warnings);
            assert!(map.is_empty());
            assert_eq!(
                vec!["Unable to set VERGEN_BUILD_DATE".to_string()],
                warnings
            );
        });
    }
}
