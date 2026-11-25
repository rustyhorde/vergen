// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use convert_case::{Case, Casing};

/// The `vergen` key whose value is emitted with escaped newlines.
const GIT_COMMIT_MESSAGE: &str = "VERGEN_GIT_COMMIT_MESSAGE";

/// Decode a value escaped by `vergen` so it could be emitted on a single
/// `cargo:rustc-env` line.
///
/// `vergen` escapes the multi-line `VERGEN_GIT_COMMIT_MESSAGE` value: each line feed
/// is written as a backslash followed by `n`, and each backslash is doubled. This
/// reverses that encoding. A backslash followed by any other character is left as-is.
///
/// [`Pretty`](crate::Pretty) calls this for you. Use it directly when reading the value
/// with [`env!`](std::env!) yourself.
///
/// ```
/// # use vergen_pretty::unescape_newlines;
/// assert_eq!(unescape_newlines(r"feat: x\n\n* detail"), "feat: x\n\n* detail");
/// assert_eq!(unescape_newlines(r"C:\\path"), r"C:\path");
/// ```
#[must_use]
pub fn unescape_newlines(value: &str) -> String {
    let mut unescaped = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => unescaped.push('\n'),
                Some('\\') | None => unescaped.push('\\'),
                Some(other) => {
                    unescaped.push('\\');
                    unescaped.push(other);
                }
            }
        } else {
            unescaped.push(c);
        }
    }
    unescaped
}

/// Join the lines of `value`, indenting every line after the first by `indent`
/// spaces so they line up under the value column. `style` is applied to each
/// line separately so styling never covers the indentation. Blank lines stay blank.
pub(crate) fn indent_continuation<F>(value: &str, indent: usize, style: F) -> String
where
    F: Fn(&str) -> String,
{
    let mut lines = value.split('\n');
    let mut output = style(lines.next().unwrap_or_default());
    for line in lines {
        output.push('\n');
        if !line.is_empty() {
            output.push_str(&" ".repeat(indent));
            output.push_str(&style(line));
        }
    }
    output
}

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
    let value = if key == GIT_COMMIT_MESSAGE {
        unescape_newlines(value)
    } else {
        value.to_string()
    };
    let key = key.to_ascii_lowercase();
    if key.starts_with("vergen") {
        let kv_vec: Vec<String> = key.split('_').filter_map(not_vergen).collect();
        Some((kv_vec, value))
    } else {
        Some((vec![key], value))
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
    use super::{has_value, indent_continuation, split_key, split_kv, unescape_newlines};
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

    #[test]
    fn unescape_newlines_works() {
        assert_eq!(unescape_newlines("subject"), "subject");
        assert_eq!(unescape_newlines(r"a\n\nb"), "a\n\nb");
        assert_eq!(unescape_newlines(r"C:\\path\\n"), r"C:\path\n");
        assert_eq!(
            unescape_newlines(r"keep \t and trailing \"),
            r"keep \t and trailing \"
        );
    }

    #[test]
    fn split_key_unescapes_commit_message() {
        let (_, value) = split_key(("VERGEN_GIT_COMMIT_MESSAGE", r"a\nb")).unwrap_or_default();
        assert_eq!("a\nb", value);
        let (_, value) = split_key(("VERGEN_GIT_BRANCH", r"a\nb")).unwrap_or_default();
        assert_eq!(r"a\nb", value);
    }

    #[test]
    fn indent_continuation_works() {
        assert_eq!(indent_continuation("one", 4, str::to_string), "one");
        assert_eq!(
            indent_continuation("one\n\ntwo", 4, str::to_string),
            "one\n\n    two"
        );
        assert_eq!(
            indent_continuation("one\ntwo", 2, |l| format!("<{l}>")),
            "<one>\n  <two>"
        );
    }
}
