// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use bincode::{
    BorrowDecode, Decode, Encode,
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
};
use bon::Builder;

use crate::{Prefix, Suffix};

/// Extension of `Pretty` to support `bincode` serialization
#[derive(Builder, Clone, Debug, Decode, Encode, PartialEq)]
pub struct PrettyExt {
    /// Environment variables from `vergen`
    vars: Vec<(String, String, String)>,
    /// Optional prefix to print before the variables
    prefix: Option<Prefix>,
    /// Optional suffix to print after the variables
    suffix: Option<Suffix>,
}

impl Encode for Prefix {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.lines.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Prefix {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Prefix::builder()
            .lines(Vec::<String>::decode(decoder)?)
            .build())
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for Prefix {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        Ok(Prefix::builder()
            .lines(Vec::<String>::borrow_decode(decoder)?)
            .build())
    }
}

impl Encode for Suffix {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.lines.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Suffix {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Suffix::builder()
            .lines(Vec::<String>::decode(decoder)?)
            .build())
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for Suffix {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        Ok(Suffix::builder()
            .lines(Vec::<String>::borrow_decode(decoder)?)
            .build())
    }
}

#[cfg(test)]
mod test {
    use crate::{Pretty, PrettyExt, vergen_pretty_env};
    use anyhow::Result;
    use bincode::{config::standard, decode_from_slice, encode_to_vec};

    #[test]
    fn pretty_encode_decode_works() -> Result<()> {
        let mut pretty = Pretty::builder().env(vergen_pretty_env!()).build();
        pretty.populate_fmt();
        let pretty_ext = PrettyExt::builder()
            .vars(pretty.vars.iter().map(|v| (v.0.clone(), v.1.clone(), v.2.clone())).collect())
            .maybe_prefix(pretty.prefix.clone())
            .maybe_suffix(pretty.suffix.clone())
            .build();
        let encoded = encode_to_vec(&pretty_ext, standard())?;
        assert!(!encoded.is_empty());
        let decoded: PrettyExt = decode_from_slice(&encoded, standard())?.0;
        assert_eq!(pretty_ext, decoded);
        Ok(())
    }
}