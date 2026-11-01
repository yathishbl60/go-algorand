// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Independent private Keccak-f rounds, without sponge or lookup binding.
mod base;
mod constraints;
mod parity;
mod position;
mod program;
mod trace;
mod values;

use core::marker::PhantomData;
use p3_field::PrimeField64;

pub use position::RowPosition;
pub use program::ProgramColumn;
pub const STATE_BITS: usize = 1600;
pub const PARITY_BITS: usize = 320;
pub const WIDTH: usize = STATE_BITS + PARITY_BITS;
pub const PREPROCESSED: usize = 11;
pub const ROUNDS: usize = 24;
pub const WORDS: usize = STATE_BITS / 16;

/// Scope and round constants are immutable and must match trusted preprocessing.
#[derive(Clone)]
pub struct CompactKeccakAir<F: PrimeField64> {
    count: usize,
    field: PhantomData<F>,
}

impl<F: PrimeField64> CompactKeccakAir<F> {
    pub fn new(permutations: usize) -> Option<Self> {
        if !(1..=1365).contains(&permutations) || F::ORDER_U64 <= 65535 {
            return None;
        }
        Some(Self {
            count: permutations,
            field: PhantomData,
        })
    }

    pub fn rows(&self) -> usize {
        (self.count * ROUNDS).next_power_of_two().max(512)
    }

    pub fn permutation_count(&self) -> usize {
        self.count
    }
}
