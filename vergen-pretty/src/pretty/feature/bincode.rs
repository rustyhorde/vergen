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

use crate::{Prefix, Suffix};

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
