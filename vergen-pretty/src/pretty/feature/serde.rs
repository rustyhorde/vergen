// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::Pretty;
use convert_case::{Case, Casing};
use serde::{
    Serialize, Serializer,
    ser::{SerializeMap, SerializeStruct},
};

pub(crate) struct VarsTuple(Vec<(String, String, String)>);

impl Serialize for Pretty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut pretty_c = self.clone();
        pretty_c.populate_fmt();

        let mut field_count = 1;

        if pretty_c.prefix.is_some() {
            field_count += 1;
        }
        if pretty_c.suffix.is_some() {
            field_count += 1;
        }

        if field_count == 1 && self.flatten {
            serializer.serialize_newtype_struct("VarsTuple", &VarsTuple(pretty_c.vars))
        } else {
            let mut state = serializer.serialize_struct("Pretty", field_count)?;
            if let Some(prefix) = pretty_c.prefix {
                state.serialize_field("prefix", &prefix)?;
            }
            state.serialize_field("vars", &VarsTuple(pretty_c.vars))?;
            if let Some(suffix) = pretty_c.suffix {
                state.serialize_field("suffix", &suffix)?;
            }
            state.end()
        }
    }
}

impl Serialize for VarsTuple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vars = &self.0;
        let mut map = serializer.serialize_map(Some(vars.len()))?;
        for (category, label, value) in vars {
            map.serialize_entry(&format!("{category}_{}", label.to_case(Case::Snake)), value)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        PrefixBuilder, PrettyBuilder, SuffixBuilder, utils::test_utils::TEST_PREFIX_SUFFIX,
        vergen_pretty_env,
    };
    use anyhow::Result;

    const VARS: &str = r#"vars":{"#;
    const PREFIX: &str = r#""prefix":{"lines":["#;
    const SUFFIX: &str = r#""suffix":{"lines":["#;

    #[test]
    fn pretty_serialize_works() -> Result<()> {
        let pretty = PrettyBuilder::default().env(vergen_pretty_env!()).build()?;
        let val = serde_json::to_string(&pretty)?;
        assert!(val.contains(VARS));
        Ok(())
    }

    #[test]
    fn pretty_with_prefix_serialize_works() -> Result<()> {
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build()?;
        let pretty = PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .build()?;
        let val = serde_json::to_string(&pretty)?;
        assert!(val.contains(VARS));
        assert!(val.contains(PREFIX));
        Ok(())
    }

    #[test]
    fn pretty_with_suffix_serialize_works() -> Result<()> {
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build()?;
        let pretty = PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .suffix(suffix)
            .build()?;
        let val = serde_json::to_string(&pretty)?;
        assert!(val.contains(VARS));
        assert!(val.contains(SUFFIX));
        Ok(())
    }

    #[test]
    fn pretty_with_flatten_serialize_works() -> Result<()> {
        let pretty = PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .flatten(true)
            .build()?;
        let val = serde_json::to_string(&pretty)?;
        assert!(!val.contains(VARS));
        assert!(!val.contains(PREFIX));
        assert!(!val.contains(SUFFIX));
        Ok(())
    }
}
