// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "bincode")]
use ::bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};
#[cfg(any(feature = "bincode", feature = "serde"))]
use {
    crate::{Prefix, Pretty, Suffix},
    bon::Builder,
};

#[cfg(feature = "bincode")]
pub(crate) mod bincode;
#[cfg(feature = "color")]
pub(crate) mod color;
#[cfg(feature = "serde")]
pub(crate) mod serde;
#[cfg(feature = "trace")]
pub(crate) mod trace;

#[cfg(any(feature = "bincode", feature = "serde"))]
/// Extension of `Pretty` to support `bincode` serialization
#[derive(Builder, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
pub struct PrettyExt {
    /// Environment variables from `vergen`
    vars: Vec<(String, String, String)>,
    /// Optional prefix to print before the variables
    prefix: Option<Prefix>,
    /// Optional suffix to print after the variables
    suffix: Option<Suffix>,
}

#[cfg(any(feature = "bincode", feature = "serde"))]
impl PrettyExt {
    /// Get the environment variables
    #[must_use]
    pub fn vars(&self) -> &Vec<(String, String, String)> {
        &self.vars
    }

    /// Get the optional prefix
    #[must_use]
    pub fn prefix(&self) -> Option<&Prefix> {
        self.prefix.as_ref()
    }

    /// Get the optional suffix
    #[must_use]
    pub fn suffix(&self) -> Option<&Suffix> {
        self.suffix.as_ref()
    }
}

#[cfg(any(feature = "bincode", feature = "serde"))]
impl From<Pretty> for PrettyExt {
    fn from(pretty: Pretty) -> Self {
        let mut pretty_c = pretty.clone();
        pretty_c.populate_fmt();
        PrettyExt::builder()
            .vars(
                pretty_c
                    .vars
                    .iter()
                    .map(|v| (v.0.clone(), v.1.clone(), v.2.clone()))
                    .collect(),
            )
            .maybe_prefix(pretty_c.prefix)
            .maybe_suffix(pretty_c.suffix)
            .build()
    }
}

#[cfg(all(test, feature = "bincode"))]
mod test_bincode {
    use crate::{
        Prefix, Pretty, PrettyExt, Suffix, utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env,
    };
    use anyhow::Result;
    use bincode::{config::standard, decode_from_slice, encode_to_vec};

    #[test]
    fn pretty_encode_decode_works() -> Result<()> {
        let pretty = Pretty::builder().env(vergen_pretty_env!()).build();
        let pretty_ext = PrettyExt::from(pretty);
        let encoded = encode_to_vec(&pretty_ext, standard())?;
        let decoded: PrettyExt = decode_from_slice(&encoded, standard())?.0;
        assert_eq!(pretty_ext, decoded);
        assert!(!decoded.vars().is_empty());
        assert!(decoded.prefix().is_none());
        assert!(decoded.suffix().is_none());
        Ok(())
    }

    #[test]
    fn pretty_encode_decode_with_prefix_suffix_works() -> Result<()> {
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let pretty = Pretty::builder()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .suffix(suffix)
            .build();
        let pretty_ext = PrettyExt::from(pretty);
        let encoded = encode_to_vec(&pretty_ext, standard())?;
        let decoded: PrettyExt = decode_from_slice(&encoded, standard())?.0;
        assert_eq!(pretty_ext, decoded);
        assert!(!decoded.vars().is_empty());
        assert!(decoded.prefix().is_some());
        assert!(decoded.suffix().is_some());
        Ok(())
    }
}

#[cfg(all(test, feature = "serde"))]
mod test_serde {
    use crate::{
        Prefix, Pretty, PrettyExt, Suffix, utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env,
    };
    use anyhow::Result;

    const VARS: &str = r#"vars":"#;
    const PREFIX: &str = r#""prefix":{"lines":["#;
    const SUFFIX: &str = r#""suffix":{"lines":["#;

    #[test]
    fn pretty_serde_works() -> Result<()> {
        let pretty = Pretty::builder().env(vergen_pretty_env!()).build();
        let pretty_ext = PrettyExt::from(pretty);
        let val = serde_json::to_string(&pretty_ext)?;
        assert!(val.contains(VARS));
        Ok(())
    }

    #[test]
    fn pretty_encode_decode_with_prefix_suffix_works() -> Result<()> {
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let pretty = Pretty::builder()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .suffix(suffix)
            .build();
        let pretty_ext = PrettyExt::from(pretty);
        let val = serde_json::to_string(&pretty_ext)?;
        assert!(val.contains(VARS));
        assert!(val.contains(PREFIX));
        assert!(val.contains(SUFFIX));
        Ok(())
    }
}
