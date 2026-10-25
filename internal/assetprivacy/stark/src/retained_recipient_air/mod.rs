// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Viewing encryption for an original-owner recovery remainder, including frozen owners.
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
pub use input::RetainedInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{FRAME_ROWS, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::RetainedWitness;

#[derive(Clone)]
pub struct RetainedRecipientAir {
    envelope: EnvelopeAir,
    view: RegisteredKeyAir,
}

impl RetainedRecipientAir {
    pub fn new(context: EnvelopeContext) -> Option<Self> {
        if context.bytes()[147] != 1 {
            return None;
        }
        Some(Self {
            envelope: EnvelopeAir::new(context)?,
            view: record(context)?,
        })
    }
}

fn record(context: EnvelopeContext) -> Option<RegisteredKeyAir> {
    let bytes = context.bytes();
    let network = bytes[43..75].try_into().ok()?;
    let asset = u64::from_be_bytes(bytes[75..83].try_into().ok()?);
    let policy = bytes[83..147].try_into().ok()?;
    RegisteredKeyAir::view(network, asset, policy, 1)
}

impl BaseAir<Value> for RetainedRecipientAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for RetainedRecipientAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_envelope_binding(builder);
        self.eval_view_binding(builder);
    }
}
