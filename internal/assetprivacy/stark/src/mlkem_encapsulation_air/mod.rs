// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private ML-KEM-768 encapsulation with the candidate's fixed matrix prefix.
//! Key authorization and its binding into the envelope remain caller obligations.
mod bind_derivation;
mod bind_matrix;
mod bind_noise;
mod binding;
mod components;
mod fixed;
mod frames;
mod layout;
mod params;
mod schedule;
mod trace;
mod writer;

use crate::Value;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
use params::*;
pub use params::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::EncapsulationWitness;

#[derive(Clone, Copy)]
pub struct MlKemEncapsulationAir;

impl BaseAir<Value> for MlKemEncapsulationAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemEncapsulationAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_fixed(builder);
        self.eval_derivation_binding(builder);
        self.eval_matrix_bindings(builder);
        self.eval_noise_bindings(builder);
    }
}
