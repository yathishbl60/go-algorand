// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed-length SHA3-512 preimage constraints, to compose with the note relation.
//! A hash preimage proof alone never authorizes a transfer.

mod absorption;
mod blocks;
mod boundary;
mod message;
mod output_bits;
mod trace;
mod trace_input;

pub use trace::digest_values;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::{KeccakAir, NUM_KECCAK_COLS};

use crate::{Value, prefix_builder::PrefixBuilder};

pub const MAX_MESSAGE_BYTES: usize = 1024;
pub(crate) const RATE_BYTES: usize = 72;

/// The verifier fixes the message length; proof data cannot choose it.
#[derive(Clone, Copy)]
pub struct Sha3Air {
    message_len: usize,
}

impl Sha3Air {
    pub fn new(message_len: usize) -> Option<Self> {
        (message_len <= MAX_MESSAGE_BYTES).then_some(Self { message_len })
    }
    pub(crate) fn blocks(&self) -> usize {
        self.message_len / RATE_BYTES + 1
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
        32
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
