use crate::{constants::VERGEN_IDEMPOTENT_DEFAULT, CargoRustcEnvMap, CargoWarning, VergenKey};
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
    doc = r"    add_default_map_entry(VergenKey::BuildDate, &mut map, &mut warning);
assert_eq!(1, map.len());
assert_eq!(1, warning.len());"
)]
/// });
/// ```
///
pub fn add_default_map_entry(
    key: VergenKey,
    map: &mut CargoRustcEnvMap,
    warnings: &mut CargoWarning,
) {
    if let Ok(value) = env::var(key.name()) {
        add_map_entry(key, value, map);
        warnings.push(format!("{} overidden", key.name()));
    } else {
        add_map_entry(key, VERGEN_IDEMPOTENT_DEFAULT, map);
        warnings.push(format!("{} set to default", key.name()));
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
