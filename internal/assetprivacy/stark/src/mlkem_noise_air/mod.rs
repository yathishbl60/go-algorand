// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! ML-KEM-768 eta=2 PRF/CBD composition, not a complete ML-KEM relation.
//! Seed, SHAKE output, selector flags and coefficients remain private.

mod binding;
mod cbd;
mod host;
mod layout;
mod trace;
mod witness;
mod writer;

use crate::{Value, shake_air::Shake256Air};
use p3_air::{Air, AirBuilder, BaseAir};

#[derive(Clone)]
pub struct MlKemNoiseAir {
    hash: Shake256Air,
    nonce: u8,
}

impl MlKemNoiseAir {
    pub fn new(nonce: u8) -> Option<Self> {
        Some(Self {
            hash: Shake256Air::new(33)?,
            nonce,
        })
    }
}

impl BaseAir<Value> for MlKemNoiseAir {
    fn width(&self) -> usize {
        self.coefficient_start() + 256
    }
    fn num_public_values(&self) -> usize {
        0
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemNoiseAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_nonce(builder);
        self.eval_output(builder);
        for i in 0..256 {
            self.eval_coefficient(builder, i);
        }
    }
}
