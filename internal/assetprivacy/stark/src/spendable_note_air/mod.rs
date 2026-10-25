// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ordinary private note spending with current hidden-account eligibility.
mod binding;
mod components;
mod fixed;
mod frames;
mod input;
mod layout;
mod messages;
mod note_binding;
mod params;
mod prepared;
mod schedule;
mod schedule_copy;
mod trace;
mod write_children;
mod writer;

use crate::{
    Value, current_eligibility_air::CurrentEligibilityAir,
    membership_vertical::MembershipVerticalAir, note_air::NoteCommitmentAir, sha3_air::Sha3Air,
};
pub use input::SpendInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{FRAME_ROWS, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::SpendNoteWitness;

#[derive(Clone)]
pub struct SpendableNoteAir {
    note: NoteCommitmentAir,
    owner: Sha3Air,
    nullifier: Sha3Air,
    membership: MembershipVerticalAir,
    eligibility: CurrentEligibilityAir,
    owner_prefix: [u8; 77],
    nullifier_prefix: [u8; 147],
}

impl SpendableNoteAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        let note = NoteCommitmentAir::new(network, asset, policy)?;
        if note.width() != SCRATCH {
            return None;
        }
        Some(Self {
            note,
            owner: Sha3Air::new(109)?,
            nullifier: Sha3Air::new(179)?,
            membership: MembershipVerticalAir::new(network, asset, policy)?,
            eligibility: CurrentEligibilityAir::new(network, asset, policy)?,
            owner_prefix: messages::owner_prefix(network),
            nullifier_prefix: messages::nullifier_prefix(network, asset, policy),
        })
    }
}

impl BaseAir<Value> for SpendableNoteAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for SpendableNoteAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_roots(builder);
        self.eval_note_binding(builder);
        self.eval_owner_binding(builder);
        self.eval_nullifier_binding(builder);
        self.eval_membership_binding(builder);
        self.eval_eligibility_binding(builder);
    }
}
