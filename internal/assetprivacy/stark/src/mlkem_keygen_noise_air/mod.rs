// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private ML-KEM-768 key-generation seed and six PRF/CBD polynomials.
//! Matrix, NTT, public-key packing and native possession remain separate.
mod binding;
mod components;
mod globals;
mod layout;
mod schedule;
mod trace;
mod writer;

use crate::{Value, mlkem_keygen_seed_air::MlKemKeygenSeedAir, mlkem_noise_air::MlKemNoiseAir};
use layout::*;
pub use layout::{PREPROCESSED, ROWS, WIDTH, Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

#[derive(Clone)]
pub struct MlKemKeygenNoiseAir {
    seed: MlKemKeygenSeedAir,
    noise: [MlKemNoiseAir; 6],
}

impl MlKemKeygenNoiseAir {
    pub fn new() -> Option<Self> {
        let seed = MlKemKeygenSeedAir::new()?;
        let noise = core::array::from_fn(|i| MlKemNoiseAir::new(i as u8).expect("fixed nonce"));
        let air = Self { seed, noise };
        air.check_shape().then_some(air)
    }
    fn check_shape(&self) -> bool {
        self.seed.width() == SEED_WIDTH
            && self
                .noise
                .iter()
                .all(|air| air.width() == NOISE_WIDTH && air.trace_rows() == 512)
    }
}

impl BaseAir<Value> for MlKemKeygenNoiseAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemKeygenNoiseAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_seed(builder);
        self.eval_noise(builder);
    }
}
