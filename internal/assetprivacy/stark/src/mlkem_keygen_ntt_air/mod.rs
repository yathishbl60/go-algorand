// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private key-generation seed/noise and all six forward NTTs.
//! This is not complete public-key possession or native registration acceptance.
mod binding;
mod components;
mod globals;
mod layout;
mod schedule;
mod trace;
mod writer;

use crate::{Value, mlkem_keygen_noise_air::MlKemKeygenNoiseAir, mlkem_ntt_air::MlKemNttAir};
pub(crate) use layout::FRAME_ROWS;
use layout::*;
pub use layout::{PREPROCESSED, ROWS, WIDTH, Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

#[derive(Clone)]
pub struct MlKemKeygenNttAir {
    noise: MlKemKeygenNoiseAir,
    ntt: MlKemNttAir,
}

impl MlKemKeygenNttAir {
    pub fn new() -> Option<Self> {
        let air = Self {
            noise: MlKemKeygenNoiseAir::new()?,
            ntt: MlKemNttAir::forward(),
        };
        (air.noise.width() == NOISE_WIDTH && air.ntt.trace_rows() == NTT_ROWS).then_some(air)
    }
}

impl BaseAir<Value> for MlKemKeygenNttAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemKeygenNttAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_transforms(builder);
    }
}
