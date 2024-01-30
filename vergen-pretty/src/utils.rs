// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use convert_case::{Case, Casing};

#[allow(clippy::ref_option_ref)]
pub(crate) fn has_value(
    tuple: (&&'static str, &Option<&'static str>),
) -> Option<(&'static str, &'static str)> {
    let (key, value) = tuple;
    if value.is_some() {
        Some((*key, value.unwrap_or_default()))
    } else {
        None
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn split_key(tuple: (&str, &str)) -> Option<(Vec<String>, String)> {
    let (key, value) = tuple;
    let key = key.to_ascii_lowercase();
    if key.starts_with("vergen") {
        let kv_vec: Vec<String> = key.split('_').filter_map(not_vergen).collect();
        Some((kv_vec, value.to_string()))
    } else {
        Some((vec![key], value.to_string()))
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn split_kv(tuple: (Vec<String>, String)) -> Option<(String, String, String)> {
    let (mut kv, v) = tuple;
    if kv.len() >= 2 {
        let category = kv.remove(0);
        let label = kv
            .into_iter()
            .map(caps_proper)
            .fold(String::new(), |a, b| a + " " + &b);
        Some((category, label, v))
    } else {
        Some(("custom".to_string(), kv[0].clone(), v))
    }
}

pub(crate) fn not_vergen(part: &str) -> Option<String> {
    if part == "vergen" {
        None
    } else {
        Some(part.to_string())
    }
}

#[allow(clippy::needless_pass_by_value)]
fn caps_proper(val: String) -> String {
    if val == "cpu" || val == "os" || val == "llvm" || val == "sha" {
        val.to_ascii_uppercase()
    } else {
        val.to_case(Case::Title)
    }
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::{has_value, split_key, split_kv};
    use std::collections::BTreeMap;

    pub(crate) const TEST_PREFIX_SUFFIX: &str = r"██████╗ ██████╗ ███████╗████████╗████████╗██╗   ██╗
██╔══██╗██╔══██╗██╔════╝╚══██╔══╝╚══██╔══╝╚██╗ ██╔╝
██████╔╝██████╔╝█████╗     ██║      ██║    ╚████╔╝ 
██╔═══╝ ██╔══██╗██╔══╝     ██║      ██║     ╚██╔╝  
██║     ██║  ██║███████╗   ██║      ██║      ██║   
╚═╝     ╚═╝  ╚═╝╚══════╝   ╚═╝      ╚═╝      ╚═╝       

4a61736f6e204f7a696173
";

    pub(crate) fn is_empty(map: &BTreeMap<&'static str, Option<&'static str>>) -> bool {
        map.iter().filter_map(has_value).count() == 0
    }

    #[test]
    fn has_value_none_is_none() {
        assert!(has_value((&"test", &None)).is_none());
    }

    #[test]
    fn split_key_no_vergen_is_not_split() {
        let (split_key, value) = split_key(("test_k", "test_v")).unwrap_or_default();
        assert_eq!(1, split_key.len());
        assert_eq!("test_k", split_key[0]);
        assert_eq!("test_v", value);
    }

    #[test]
    fn split_kv_too_short_is_custom() {
        let (category, label, value) =
            split_kv((vec!["test_k".to_string()], "test_v".to_string())).unwrap_or_default();
        assert_eq!("custom", category);
        assert_eq!("test_k", label);
        assert_eq!("test_v", value);
    }
}
