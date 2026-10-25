// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recipient output encryption bound to current keys and effective eligibility.
//! Disclosure policy, spending and native verification remain separate.
mod authorization;
mod binding;
mod components;
mod eligibility_binding;
mod fixed;
mod frames;
mod input;
mod layout;
mod params;
mod schedule;
mod trace;

use crate::{
    Value,
    current_eligibility_air::CurrentEligibilityAir,
    envelope_air::{EnvelopeAir, EnvelopeContext},
    registered_key_air::RegisteredKeyAir,
};
pub use input::RecipientInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::RecipientWitness;

#[derive(Clone)]
pub struct AuthorizedRecipientAir {
    envelope: EnvelopeAir,
    view: RegisteredKeyAir,
    spend: RegisteredKeyAir,
    eligibility: CurrentEligibilityAir,
}

impl AuthorizedRecipientAir {
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
            eligibility: CurrentEligibilityAir::new(network, asset, policy)?,
        })
    }
}

impl BaseAir<Value> for AuthorizedRecipientAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for AuthorizedRecipientAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_envelope_binding(builder);
        self.eval_view_binding(builder);
        self.eval_spend_binding(builder);
        self.eval_eligibility_binding(builder);
    }
}
