// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Two private inputs and outputs with exact integer conservation and group context.
//! Ciphertext correctness, active controls, and ledger freshness remain separate.

mod binding;
mod builder;
mod components;
mod public;
mod trace;
mod witness;

pub use public::{InputPublic, PublicStatement};
pub use witness::{InputWitness, TransferWitness};

use crate::note_air::NoteCommitmentAir;
use crate::{Value, amounts::PublicValue, member_note_air::MemberNoteAir};
use p3_air::{Air, AirBuilder, BaseAir};

pub const PUBLIC_VALUES: usize = 232;
const PUBLIC_INPUT_WIDTH: usize = 64;
const OUTPUT_START: usize = 128;
const CONVERSION_START: usize = 192;
const CONTEXT_START: usize = 200;

#[derive(Clone)]
pub struct TransferAir {
    input: MemberNoteAir,
    output: NoteCommitmentAir,
    conversion: PublicValue,
}

impl TransferAir {
    pub fn new(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        conversion: PublicValue,
    ) -> Option<Self> {
        Some(Self {
            input: MemberNoteAir::new(network, asset, policy)?,
            output: NoteCommitmentAir::new(network, asset, policy)?,
            conversion,
        })
    }
    fn carry_start(&self) -> usize {
        2 * self.input.width() + 2 * self.output.width()
    }
    fn width(&self) -> usize {
        self.carry_start() + 3
    }
}

impl BaseAir<Value> for TransferAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for TransferAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_conservation(builder);
        self.eval_conversion(builder);
    }
}
