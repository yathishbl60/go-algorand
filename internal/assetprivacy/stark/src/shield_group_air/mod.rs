// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Staged two-output public shielding with no private input or dummy witness.
mod binding;
mod components;
mod confidential;
mod params;
mod public;
mod schedule;
mod trace;

use crate::{Value, amounts::PublicValue, note_air::NoteCommitmentAir};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{CONFIDENTIAL_PUBLIC_VALUES, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use public::PublicStatement;
pub use trace::GroupWitness;

#[derive(Clone)]
pub struct ShieldGroupAir {
    output: NoteCommitmentAir,
    debit: u64,
    participants: Option<[[u8; 32]; 4]>,
}

impl ShieldGroupAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64], debit: u64) -> Option<Self> {
        let output = NoteCommitmentAir::new(network, asset, policy)?;
        (debit != 0 && output.width() == NOTE_WIDTH).then_some(Self {
            output,
            debit,
            participants: None,
        })
    }
    fn conversion(&self) -> PublicValue {
        PublicValue {
            debit: self.debit,
            credit: 0,
        }
    }
}

impl BaseAir<Value> for ShieldGroupAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        self.public_count()
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for ShieldGroupAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_amounts(builder);
        self.eval_public(builder);
        self.eval_output_amounts(builder);
        self.eval_participants(builder);
    }
}
