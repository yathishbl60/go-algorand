// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private effective-unfrozen account membership under the current eligibility root.
mod binding;
mod components;
mod fixed;
mod frames;
mod layout;
mod params;
mod prepared;
mod record_writer;
mod schedule;
mod trace;
mod writer;

use crate::{
    Value, eligible_record_air::EligibleRecordAir, membership_vertical::MembershipVerticalAir,
};
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{FRAME_ROWS, PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::{EligibilityInput, EligibilityWitness};

#[derive(Clone)]
pub struct CurrentEligibilityAir {
    record: EligibleRecordAir,
    membership: MembershipVerticalAir,
}

impl CurrentEligibilityAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        Some(Self {
            record: EligibleRecordAir::new(network, asset, policy)?,
            membership: MembershipVerticalAir::new(network, asset, policy)?,
        })
    }
}

impl BaseAir<Value> for CurrentEligibilityAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for CurrentEligibilityAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_fixed(builder);
        self.eval_record_binding(builder);
        self.eval_membership_binding(builder);
    }
}
