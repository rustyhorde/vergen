use crate::{constants::VERGEN_IDEMPOTENT_DEFAULT, CargoRustcEnvMap, CargoWarning, VergenKey};
use std::env;

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

pub fn add_map_entry<T>(key: VergenKey, value: T, map: &mut CargoRustcEnvMap)
where
    T: Into<String>,
{
    let _old = map.insert(key, value.into());
}

pub fn count_idempotent(map: &CargoRustcEnvMap) -> usize {
    map.values()
        .filter(|x| *x == VERGEN_IDEMPOTENT_DEFAULT)
        .count()
}
