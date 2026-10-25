// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical recovery input and shared spent marker; issuer authority is external.
mod binding;
mod components;
mod fixed;
mod frames;
mod input;
mod layout;
mod message;
mod note_binding;
mod nullifier_binding;
mod params;
mod prepared;
mod schedule;
mod trace;
mod write_children;
mod writer;

use crate::{
    Value, membership_vertical::MembershipVerticalAir, note_air::NoteCommitmentAir,
    sha3_air::Sha3Air,
};
pub use input::RecoveryInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{FRAME_ROWS, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::RecoveryNoteWitness;

#[derive(Clone)]
pub struct RecoveryNoteAir {
    note: NoteCommitmentAir,
    nullifier: Sha3Air,
    membership: MembershipVerticalAir,
    prefix: [u8; 149],
}

impl RecoveryNoteAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        let note = NoteCommitmentAir::new(network, asset, policy)?;
        if note.width() != SCRATCH {
            return None;
        }
        Some(Self {
            note,
            nullifier: Sha3Air::new(crate::nullifier_v2::MESSAGE_BYTES)?,
            membership: MembershipVerticalAir::new(network, asset, policy)?,
            prefix: message::prefix(network, asset, policy),
        })
    }
}

impl BaseAir<Value> for RecoveryNoteAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build(self))
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for RecoveryNoteAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_roots(builder);
        self.eval_note_binding(builder);
        self.eval_nullifier_binding(builder);
        self.eval_nullifier_commitment(builder);
        self.eval_membership_binding(builder);
    }
}
