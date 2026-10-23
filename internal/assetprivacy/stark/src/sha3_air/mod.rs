// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed-length SHA3-256/512 constraints, to compose with the complete relation.
//! A hash preimage proof alone never authorizes a transfer.

mod absorption;
mod blocks;
mod boundary;
mod message;
mod output_bits;
mod shape;
mod trace;
mod trace_input;

pub use trace::{digest_values, digest256_values};

use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::{KeccakAir, NUM_KECCAK_COLS};

use crate::{Value, prefix_builder::PrefixBuilder};

pub const MAX_MESSAGE_BYTES: usize = 4096;

/// The verifier fixes the message length; proof data cannot choose it.
#[derive(Clone, Copy)]
pub struct Sha3Air {
    message_len: usize,
    digest_bytes: usize,
}

impl Sha3Air {
    pub fn new(message_len: usize) -> Option<Self> {
        Self::with_digest_length(message_len, 64)
    }
    pub fn new256(message_len: usize) -> Option<Self> {
        Self::with_digest_length(message_len, 32)
    }
    pub(crate) fn blocks(&self) -> usize {
        self.message_len / self.rate_bytes() + 1
    }
    pub(crate) fn block_offset(&self) -> usize {
        NUM_KECCAK_COLS + self.message_len * 8
    }
    pub(crate) fn width(&self) -> usize {
        self.block_offset() + self.blocks()
    }
}

impl BaseAir<Value> for Sha3Air {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        self.digest_bytes / 2
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for Sha3Air {
    fn eval(&self, builder: &mut AB) {
        KeccakAir {}.eval(&mut PrefixBuilder {
            inner: builder,
            width: NUM_KECCAK_COLS,
        });
        self.eval_message(builder);
        self.eval_blocks(builder);
        self.eval_initial(builder);
        self.eval_absorption(builder);
        self.eval_digest(builder);
    }
}
