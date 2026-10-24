// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete fixed-envelope GCM in one hiding AIR with staged scratch reuse.
mod binding;
pub(crate) mod frame;
pub(crate) mod frame_builder;
mod frame_schedule;
mod layout;
mod schedule;
mod trace;
mod writer;

use crate::{Value, aes_gcm_vertical::AesGcmAir, ghash_chain::GhashChainAir};
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use trace::GcmWitness;

pub const ROWS: usize = 8192;
pub(crate) const FRAME_ROWS: usize = 5249;
pub const WIDTH: usize = 11728;
pub const PUBLIC_VALUES: usize = 276;
const AES_WIDTH: usize = 11216;
const AES_SCHEDULE: usize = 61;
const GH_SCHEDULE: usize = 165;
const GH_START: usize = 1920;
const GH_ROW: usize = 512;
const PT: usize = 6768;
const H: usize = 10960;
const MASK: usize = 11088;
const AAD: usize = 11216;
const AES_FLAGS: usize = 226;
const GH_FLAGS: usize = 230;
const PREPROCESSED: usize = 234;

#[derive(Clone, Copy)]
pub struct GcmVerticalAir;

impl BaseAir<Value> for GcmVerticalAir {
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
        Some(schedule::schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for GcmVerticalAir {
    fn eval(&self, builder: &mut AB) {
        AesGcmAir.eval(&mut frame_builder::FrameBuilder::new(builder, frame::aes()));
        GhashChainAir.eval(&mut frame_builder::FrameBuilder::new(
            builder,
            frame::ghash(),
        ));
        self.eval_constants(builder);
        self.eval_hash_bindings(builder);
        self.eval_tag(builder);
    }
}
