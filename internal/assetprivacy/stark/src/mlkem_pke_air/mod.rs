// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete private K-PKE arithmetic given matrix/noise/key/message inputs.
//! The enclosing KEM must still prove their derivation and authorization.
mod addition;
mod addition_binding;
mod binding;
mod components;
mod frames;
mod layout;
mod params;
mod plan;
mod prepared;
mod schedule;
mod schedule_row;
mod trace;
mod write_add;
mod write_fixed;
mod writer;

use crate::Value;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use trace::PkeWitness;

use params::*;
pub use params::{PUBLIC_VALUES, ROWS, WIDTH};

#[derive(Clone, Copy)]
pub struct MlKemPkeAir;

impl BaseAir<Value> for MlKemPkeAir {
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
        Some(schedule::build())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemPkeAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_ntt_bindings(builder);
        self.eval_raw_carry(builder);
        self.eval_additions(builder);
    }
}
