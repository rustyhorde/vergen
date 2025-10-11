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
    use crate::{Pretty, PrettyExt, vergen_pretty_env};
    use anyhow::Result;
    use bincode::{config::standard, decode_from_slice, encode_to_vec};

    #[test]
    fn pretty_encode_decode_works() -> Result<()> {
        let pretty = Pretty::builder().env(vergen_pretty_env!()).build();
        let pretty_ext = PrettyExt::from(pretty);
        let encoded = encode_to_vec(&pretty_ext, standard())?;
        let decoded: PrettyExt = decode_from_slice(&encoded, standard())?.0;
        assert_eq!(pretty_ext, decoded);
        Ok(())
    }
}

#[cfg(all(test, feature = "serde"))]
mod test_serde {
    use crate::{Pretty, PrettyExt, vergen_pretty_env};
    use anyhow::Result;

    #[test]
    fn pretty_serde_works() -> Result<()> {
        let pretty = Pretty::builder().env(vergen_pretty_env!()).build();
        let pretty_ext = PrettyExt::from(pretty);
        let val = serde_json::to_string(&pretty_ext)?;
        assert!(val.contains(r#"{"vars":"#));
        Ok(())
    }
}
