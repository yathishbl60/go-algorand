// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Current recipient keys for an issuer-recovery receipt, including frozen accounts.
//! Current native issuer/PQ authentication remains mandatory outside this AIR.
mod binding;
mod components;
mod fixed;
mod frames;
mod input;
mod layout;
mod params;
mod schedule;
mod trace;

use crate::{
    Value,
    envelope_air::{EnvelopeAir, EnvelopeContext},
    registered_key_air::RegisteredKeyAir,
};
pub use input::RecoveryRecipientInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::RecoveryRecipientWitness;

#[derive(Clone)]
pub struct RecoveryRecipientAir {
    envelope: EnvelopeAir,
    view: RegisteredKeyAir,
    spend: RegisteredKeyAir,
}

impl RecoveryRecipientAir {
    pub fn new(context: EnvelopeContext) -> Option<Self> {
        let bytes = context.bytes();
        if bytes[147] != 1 {
            return None;
        }
        let network = bytes[43..75].try_into().ok()?;
        let asset = u64::from_be_bytes(bytes[75..83].try_into().ok()?);
        let policy = bytes[83..147].try_into().ok()?;
        Some(Self {
            envelope: EnvelopeAir::new(context)?,
            view: RegisteredKeyAir::view(network, asset, policy, 1)?,
            spend: RegisteredKeyAir::spend(network, asset, policy)?,
        })
    }
}

impl BaseAir<Value> for RecoveryRecipientAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for RecoveryRecipientAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_envelope_binding(builder);
        self.eval_view_binding(builder);
        self.eval_spend_binding(builder);
    }
}
