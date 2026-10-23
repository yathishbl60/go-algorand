// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Two-output public-to-private conservation; no private inputs or dummy notes.
//! Encryption correctness and native debit/policy/funding checks remain separate.

mod binding;
mod public;
mod trace;

pub use public::PublicStatement;

use crate::{Value, note_air::NoteCommitmentAir};
use p3_air::{Air, AirBuilder, BaseAir};

pub const PUBLIC_VALUES: usize = 100;
const DEBIT_START: usize = 64;
const CONTEXT_START: usize = 68;

#[derive(Clone)]
pub struct ShieldAir {
    output: NoteCommitmentAir,
    debit: u64,
}

impl ShieldAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64], debit: u64) -> Option<Self> {
        Some(Self {
            output: NoteCommitmentAir::new(network, asset, policy)?,
            debit,
        })
    }
    fn auxiliary_start(&self) -> usize {
        2 * self.output.width()
    }
    fn width(&self) -> usize {
        self.auxiliary_start() + 4
    }
}

impl BaseAir<Value> for ShieldAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for ShieldAir {
    fn eval(&self, builder: &mut AB) {
        for index in 0..2 {
            self.eval_output(builder, index);
        }
        self.eval_conservation(builder);
        self.eval_debit(builder);
    }
}
