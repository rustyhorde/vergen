// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "color")]
use console::Style;
use indexmap::IndexSet;
use lazy_static::lazy_static;

pub(crate) mod display;
#[cfg(feature = "trace")]
pub(crate) mod trace;

lazy_static! {
    pub(crate) static ref VERGEN_MAP: IndexSet<(&'static str, &'static str, Option<&'static str>)> = {
        let mut vergen_set = IndexSet::new();
        let _ = vergen_set.insert(("Version", "build", option_env!("CARGO_PKG_VERSION")));
        // build output
        let _ = vergen_set.insert((
            "Timestamp",
            "build",
            option_env!("VERGEN_BUILD_DATE"),
        ));
        let _ = vergen_set.insert((
            "Timestamp",
            "build",
            option_env!("VERGEN_BUILD_TIMESTAMP"),
        ));
        // git output
        let _ = vergen_set.insert(("Branch", "git", option_env!("VERGEN_GIT_BRANCH")));
        let _ = vergen_set.insert(("Commit Author Email", "git", option_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL")));
        let _ = vergen_set.insert(("Commit Author Name", "git", option_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME")));
        let _ = vergen_set.insert(("Commit Count", "git", option_env!("VERGEN_GIT_COMMIT_COUNT")));
        let _ = vergen_set.insert(("Commit Date", "git", option_env!("VERGEN_GIT_COMMIT_DATE")));
        let _ = vergen_set.insert(("Commit Message", "git", option_env!("VERGEN_GIT_COMMIT_MESSAGE")));
        let _ = vergen_set.insert(("Commit Timestamp", "git", option_env!("VERGEN_GIT_COMMIT_TIMESTAMP")));
        let _ = vergen_set.insert(("Describe", "git", option_env!("VERGEN_GIT_DESCRIBE")));
        let _ = vergen_set.insert(("Commit SHA", "git", option_env!("VERGEN_GIT_SHA")));
        // rustc output
        let _ = vergen_set.insert(("Semver", "rustc", option_env!("VERGEN_RUSTC_SEMVER")));
        let _ = vergen_set.insert(("Channel", "rustc", option_env!("VERGEN_RUSTC_CHANNEL")));
        let _ = vergen_set.insert((
            "Host Triple",
            "rustc",
            option_env!("VERGEN_RUSTC_HOST_TRIPLE"),
        ));
        let _ = vergen_set.insert((
            "LLVM Version",
            "rustc",
            option_env!("VERGEN_RUSTC_LLVM_VERSION"),
        ));
        let _ = vergen_set.insert((
            "Commit Date",
            "rustc",
            option_env!("VERGEN_RUSTC_COMMIT_DATE"),
        ));
        let _ = vergen_set.insert((
            "Commit SHA",
            "rustc",
            option_env!("VERGEN_RUSTC_COMMIT_HASH"),
        ));
        // cargo output
        let _ = vergen_set.insert(("Debug", "cargo", option_env!("VERGEN_CARGO_DEBUG")));
        let _ = vergen_set.insert(("Features", "cargo", option_env!("VERGEN_CARGO_FEATURES")));
        let _ = vergen_set.insert(("OptLevel", "cargo", option_env!("VERGEN_CARGO_OPT_LEVEL")));
        let _ = vergen_set.insert((
            "Target Triple",
            "cargo",
            option_env!("VERGEN_CARGO_TARGET_TRIPLE"),
        ));
        // sysinfo output
        let _ = vergen_set.insert(("Name", "sysinfo", option_env!("VERGEN_SYSINFO_NAME")));
        let _ = vergen_set.insert((
            "OS Version",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_OS_VERSION"),
        ));
        let _ = vergen_set.insert(("User", "sysinfo", option_env!("VERGEN_SYSINFO_USER")));
        let _ = vergen_set.insert((
            "Memory",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_TOTAL_MEMORY"),
        ));
        let _ = vergen_set.insert((
            "CPU Vendor",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_CPU_VENDOR"),
        ));
        let _ = vergen_set.insert((
            "CPU Cores",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_CPU_CORE_COUNT"),
        ));
        let _ = vergen_set.insert((
            "CPU Names",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_CPU_NAME"),
        ));
        let _ = vergen_set.insert((
            "CPU Brand",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_CPU_BRAND"),
        ));
        let _ = vergen_set.insert((
            "CPU Frequency",
            "sysinfo",
            option_env!("VERGEN_SYSINFO_CPU_FREQUENCY"),
        ));
        vergen_set
    };
}

#[cfg(feature = "color")]
lazy_static! {
    pub(crate) static ref BOLD_BLUE: Style = Style::new().bold().blue();
    pub(crate) static ref BOLD_GREEN: Style = Style::new().bold().green();
}

fn has_value(
    tuple: &(&'static str, &'static str, Option<&'static str>),
) -> Option<(&'static str, &'static str, &'static str)> {
    let (prefix, kind, value) = tuple;
    if value.is_some() {
        Some((*prefix, *kind, value.unwrap_or_default()))
    } else {
        None
    }
}

fn determine_maxes() -> (
    impl Iterator<Item = (&'static str, &'static str, &'static str)>,
    usize,
    usize,
) {
    let vm_iter = (*VERGEN_MAP).iter().filter_map(has_value);
    let max_prefix = vm_iter
        .clone()
        .map(|(prefix, _, _)| prefix.len())
        .max()
        .map_or_else(|| 16, |x| x);
    let max_kind = vm_iter
        .clone()
        .map(|(_, kind, _)| kind.len())
        .max()
        .map_or_else(|| 7, |x| x);
    (vm_iter, max_prefix, max_kind)
}

#[cfg(test)]
mod test {
    use super::has_value;

    #[test]
    fn none_env_is_none() {
        assert!(has_value(&("test", "test", None)).is_none());
    }

    #[test]
    fn some_env_is_some() {
        assert!(has_value(&("test", "test", Some("test"))).is_some());
    }
}
