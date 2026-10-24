// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! HMAC-SHA3-512 with a private 64-byte key and fixed-length private data.
//! This is the key-size profile needed by the candidate HKDF, not generic HMAC.

mod binding;
mod components;
mod layout;
mod prepared;
mod trace;
mod trace_join;
mod value;
mod writer;
pub(crate) use prepared::derive_output;

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};

pub const MAX_DATA_BYTES: usize = 1024;
const BLOCK_BYTES: usize = 72;

#[derive(Clone)]
pub struct HmacAir {
    inner: Sha3Air,
    outer: Sha3Air,
    data_len: usize,
}

impl HmacAir {
    pub fn new(data_len: usize) -> Option<Self> {
        if data_len > MAX_DATA_BYTES {
            return None;
        }
        Some(Self {
            inner: Sha3Air::new(BLOCK_BYTES + data_len)?,
            outer: Sha3Air::new(BLOCK_BYTES + 64)?,
            data_len,
        })
    }
}

impl BaseAir<Value> for HmacAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        0
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for HmacAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_keys(builder);
        self.eval_inner_digest(builder);
        self.eval_output(builder);
    }
}
