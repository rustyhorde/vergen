// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::determine_maxes;
#[cfg(feature = "color")]
use super::{BOLD_BLUE, BOLD_GREEN};
use tracing::info;

/// Output the `vergen` environment variables in table format as trace info
pub fn trace() {
    let (vm_iter, max_prefix, max_kind) = determine_maxes();
    for (prefix, kind, value) in vm_iter {
        let key = format!("{prefix:>max_prefix$} ({kind:>max_kind$})");
        inner_trace(&key, value);
    }
}

#[cfg(feature = "color")]
fn inner_trace(key: &str, value: &str) {
    let blue_key = (*BOLD_BLUE).apply_to(key);
    let green_val = (*BOLD_GREEN).apply_to(value);
    info!("{blue_key} {green_val}");
}

#[cfg(not(feature = "color"))]
fn inner_trace(key: &str, value: &str) {
    info!("{key} {value}");
}

#[cfg(test)]
mod tests {
    use super::trace;

    #[test]
    fn trace_works() {
        trace();
    }
}
