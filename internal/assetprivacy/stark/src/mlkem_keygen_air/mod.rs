// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact public ML-KEM-768 key generation from a private derivation seed.
//! Native registration authorization/artifact selection remains separate.
mod binding;
mod components;
mod context;
mod layout;
mod public;
mod schedule;
mod trace;
mod witness;

use crate::{Value, mlkem_dot_air::MlKemDotAir, mlkem_keygen_ntt_air::MlKemKeygenNttAir};
use context::PublicKey;
use layout::*;
pub use layout::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use public::PublicStatement;
pub use trace::GroupWitness;

#[derive(Clone)]
pub struct MlKemKeygenAir {
    key: PublicKey,
    core: MlKemKeygenNttAir,
    dot: MlKemDotAir,
}

impl MlKemKeygenAir {
    pub fn new(public_key: [u8; 1184]) -> Option<Self> {
        Some(Self {
            key: PublicKey::parse(public_key)?,
            core: MlKemKeygenNttAir::new()?,
            dot: MlKemDotAir,
        })
    }
}

impl BaseAir<Value> for MlKemKeygenAir {
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
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemKeygenAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_rho(builder);
        self.eval_dot_inputs(builder);
        self.eval_sum(builder);
        self.eval_public(builder);
    }
}
