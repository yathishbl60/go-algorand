// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private key record membership under an authoritative current registry root.
mod bind_spend;
mod bind_view;
mod binding;
mod components;
mod fixed;
mod frames;
mod layout;
mod params;
mod prepared;
mod record;
mod schedule;
mod trace;
mod write_record;
mod writer;

use crate::{Value, membership_vertical::MembershipVerticalAir};
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
use record::RecordAir;
pub use trace::{KeyMaterial, RegisteredKeyInput, RegisteredKeyWitness};

#[derive(Clone)]
pub struct RegisteredKeyAir {
    record: RecordAir,
    membership: MembershipVerticalAir,
}

impl RegisteredKeyAir {
    pub fn view(network: [u8; 32], asset: u64, policy: [u8; 64], role: u8) -> Option<Self> {
        let record = crate::view_key_air::ViewKeyAir::new(network, asset, policy, role)?;
        Self::new(RecordAir::View(record), network, asset, policy)
    }
    pub fn spend(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        let record = crate::spend_key_air::SpendKeyAir::new(network, asset, policy)?;
        Self::new(RecordAir::Spend(record), network, asset, policy)
    }
    fn new(record: RecordAir, network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        Some(Self {
            record,
            membership: MembershipVerticalAir::new(network, asset, policy)?,
        })
    }
    pub(crate) fn frame_rows(&self) -> usize {
        self.record.rows() + crate::membership_vertical::FRAME_ROWS
    }
}

impl BaseAir<Value> for RegisteredKeyAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for RegisteredKeyAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_fixed(builder);
        self.eval_record_binding(builder);
        self.eval_membership_binding(builder);
    }
}
