// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Staged ordinary holder inputs, current eligibility and exact group conservation.
//! Native policy/ledger checks and required output-envelope proofs remain separate.
mod binding;
mod components;
mod frames;
mod input;
mod mapped_builder;
mod operation;
mod params;
mod prepared;
mod public;
mod schedule;
mod trace;
mod writer;

use crate::{Value, note_air::NoteCommitmentAir, spendable_note_air::SpendableNoteAir};
pub use input::GroupInput;
pub use operation::Operation;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, ROWS, TRANSFER_PUBLIC_VALUES, UNSHIELD_PUBLIC_VALUES, WIDTH};
pub use public::{InputPublic, PublicStatement};
pub use trace::GroupWitness;

#[derive(Clone)]
pub struct SpendGroupAir {
    input: SpendableNoteAir,
    output: NoteCommitmentAir,
    operation: Operation,
}

impl SpendGroupAir {
    pub fn new(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        operation: Operation,
    ) -> Option<Self> {
        let output = NoteCommitmentAir::new(network, asset, policy)?;
        if output.width() > crate::spendable_note_air::WIDTH {
            return None;
        }
        Some(Self {
            input: SpendableNoteAir::new(network, asset, policy)?,
            output,
            operation,
        })
    }
}

impl BaseAir<Value> for SpendGroupAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        self.operation.public_count()
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build(self))
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for SpendGroupAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_amounts(builder);
        self.eval_conversion(builder);
        self.eval_bindings(builder);
    }
}
