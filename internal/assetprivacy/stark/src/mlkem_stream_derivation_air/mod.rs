// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private canonical ML-KEM key hashing and encapsulation-coin derivation.
mod absorption;
mod boundary;
mod canonical;
mod fixed;
mod layout;
mod message;
mod schedule;
mod states;
mod trace;
mod windows;
mod witness;
mod write;
mod writer;

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::{KeccakAir, NUM_KECCAK_COLS};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 1024;
pub const WIDTH: usize = 5810;
pub const PREPROCESSED: usize = 792;
const RHO: usize = 768;
const MESSAGE: usize = 1024;
const HASH: usize = 1280;
const OUTPUT: usize = 1536;
const KECCAK: usize = 2048;
const WINDOW: usize = KECCAK + NUM_KECCAK_COLS;
const CANONICAL: usize = WINDOW + 1104;
const BLOCK: usize = 768;
const END: usize = 777;
const FRAME: usize = 785;
const H_END: usize = 789;
const G_END: usize = 790;
const G_FIRST: usize = 791;

#[derive(Clone, Copy)]
pub struct MlKemStreamDerivationAir;

impl MlKemStreamDerivationAir {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for MlKemStreamDerivationAir {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseAir<Value> for MlKemStreamDerivationAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemStreamDerivationAir {
    fn eval(&self, builder: &mut AB) {
        use crate::gcm_vertical::{frame::Frame, frame_builder::FrameBuilder};
        let frame = Frame {
            columns: KECCAK..WINDOW,
            preprocessing: 0..0,
            public: 0..0,
            flags: FRAME,
        };
        KeccakAir {}.eval(&mut FrameBuilder::new(builder, frame));
        self.eval_fixed(builder);
        self.eval_canonical(builder);
        self.eval_windows(builder);
        self.eval_initial(builder);
        self.eval_absorption(builder);
        self.eval_digest(builder);
    }
}
