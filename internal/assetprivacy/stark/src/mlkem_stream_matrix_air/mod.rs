// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete private fixed-prefix SHAKE128 and ML-KEM SampleNTT relation.
mod binding;
mod boundary;
mod cursor;
mod layout;
mod sampling;
mod schedule;
mod trace;
mod writer;
mod witness;

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::{KeccakAir, NUM_KECCAK_COLS};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 4096;
pub const WIDTH: usize = 4872;
pub const PREPROCESSED: usize = 118;
const BLOCK_ROWS: usize = 136;
const BLOCKS: usize = 16;
const ACTIVE_ROWS: usize = BLOCKS * BLOCK_ROWS;
const SEED: usize = NUM_KECCAK_COLS;
const BUFFER: usize = SEED + 256;
const RATE: usize = BUFFER + 100;
const OUTPUT: usize = RATE + 1344;
const CURSOR: usize = OUTPUT + 256;
const CANDIDATE: usize = CURSOR + 257;
const GAP: usize = CANDIDATE + 12;
const REJECT: usize = GAP + 12;
const ACCEPT: usize = REJECT + 1;
const FRAME: usize = 112;
const LATER: usize = 116;
const RATE_CARRY: usize = 117;

#[derive(Clone, Copy)]
pub struct MlKemStreamMatrixAir {
    first: u8,
    second: u8,
}

impl MlKemStreamMatrixAir {
    pub const fn new(first: u8, second: u8) -> Option<Self> {
        if first >= 3 || second >= 3 { return None; }
        Some(Self { first, second })
    }
}

impl BaseAir<Value> for MlKemStreamMatrixAir {
    fn width(&self) -> usize { WIDTH }
    fn preprocessed_width(&self) -> usize { PREPROCESSED }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemStreamMatrixAir {
    fn eval(&self, builder: &mut AB) {
        use crate::gcm_vertical::{frame::Frame, frame_builder::FrameBuilder};
        let frame = Frame { columns: 0..SEED, preprocessing: 0..0, public: 0..0, flags: FRAME };
        KeccakAir {}.eval(&mut FrameBuilder::new(builder, frame));
        self.eval_fixed(builder);
        self.eval_initial(builder);
        self.eval_buffer(builder);
        self.eval_rate(builder);
        self.eval_candidate(builder);
        self.eval_comparison(builder);
        self.eval_cursor(builder);
        self.eval_output(builder);
    }
}
