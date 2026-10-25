// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Authorized issuer-recovery arithmetic; native current-authority signatures are mandatory.
mod amounts;
mod authority;
mod binding;
mod checks;
mod components;
mod constants;
mod fixed;
mod frames;
mod input;
mod operation;
mod params;
mod prepared;
mod preservation;
mod public;
mod schedule;
mod trace;
mod writer;

use crate::{Value, note_air::NoteCommitmentAir, recovery_note_air::RecoveryNoteAir};
pub use input::GroupInput;
pub use operation::Operation;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, ROWS, WIDTH};
pub use public::{InputPublic, PublicStatement};
pub use trace::GroupWitness;

#[derive(Clone)]
pub struct RecoveryGroupAir {
    input: RecoveryNoteAir,
    output: NoteCommitmentAir,
    operation: Operation,
    authority: [u8; 32],
}

impl RecoveryGroupAir {
    pub fn new(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        operation: Operation,
        authority: [u8; 32],
    ) -> Option<Self> {
        if authority == [0; 32] || !operation.valid() {
            return None;
        }
        Some(Self {
            input: RecoveryNoteAir::new(network, asset, policy)?,
            output: NoteCommitmentAir::new(network, asset, policy)?,
            operation,
            authority,
        })
    }
}

impl BaseAir<Value> for RecoveryGroupAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for RecoveryGroupAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_amounts(builder);
        self.eval_authority(builder);
        self.eval_conversion(builder);
        self.eval_bindings(builder);
        self.eval_preservation(builder);
    }
}
