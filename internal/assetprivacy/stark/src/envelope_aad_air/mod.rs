// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed envelope AAD hash with private canonical key and public KEM ciphertext.
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
pub const WIDTH: usize = 4846;
pub const PREPROCESSED: usize = 841;
pub const PUBLIC_VALUES: usize = 544;
const RHO: usize = 768;
const OUTPUT: usize = 1024;
const EPOCH: usize = 1536;
const KECCAK: usize = 1600;
const WINDOW: usize = KECCAK + NUM_KECCAK_COLS;
const CANONICAL: usize = WINDOW + 588;
const KEY_BLOCK: usize = 768;
const CT_BLOCK: usize = 785;
const END: usize = 801;
const FRAME: usize = 836;
const DIGEST: usize = 840;

#[derive(Clone)]
pub struct EnvelopeAadAir {
    context: [u8; 248],
}

impl EnvelopeAadAir {
    pub const fn new(context: [u8; 248]) -> Self {
        Self { context }
    }
}

impl BaseAir<Value> for EnvelopeAadAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for EnvelopeAadAir {
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
        self.eval_key_windows(builder);
        self.eval_ciphertext_windows(builder);
        self.eval_initial(builder);
        self.eval_absorption(builder);
        self.eval_digest(builder);
    }
}
