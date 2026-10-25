// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Auditor/controller encryption for an authoritative public assignment.
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
pub use input::DisclosureInput;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{FRAME_ROWS, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::DisclosureWitness;

#[derive(Clone)]
pub struct AuthorizedDisclosureAir {
    envelope: EnvelopeAir,
    view: RegisteredKeyAir,
    authority: [u8; 32],
}

impl AuthorizedDisclosureAir {
    pub fn new(context: EnvelopeContext, authority: [u8; 32]) -> Option<Self> {
        if authority == [0; 32] {
            return None;
        }
        let role = match context.bytes()[147] {
            2 => 3,
            3 => 4,
            _ => return None,
        };
        Some(Self {
            envelope: EnvelopeAir::new(context)?,
            view: record(context, role)?,
            authority,
        })
    }
}

fn record(context: EnvelopeContext, role: u8) -> Option<RegisteredKeyAir> {
    let bytes = context.bytes();
    let network = bytes[43..75].try_into().ok()?;
    let asset = u64::from_be_bytes(bytes[75..83].try_into().ok()?);
    let policy = bytes[83..147].try_into().ok()?;
    RegisteredKeyAir::view(network, asset, policy, role)
}

impl BaseAir<Value> for AuthorizedDisclosureAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for AuthorizedDisclosureAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_authority(builder);
        self.eval_envelope_binding(builder);
        self.eval_view_binding(builder);
    }
}
