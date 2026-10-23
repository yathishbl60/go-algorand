// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed candidate GHASH chain with private H/AAD/result and public ciphertext.
//! Trusted preprocessing is required; this does not authorize encryption.

mod binding;
mod data;
mod schedule;
mod trace;
mod transition;
mod wires;
mod witness_state;
mod witness_write;

pub use wires::{Wires, wires};

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 8192;
pub const WIDTH: usize = 1537;
pub const PUBLIC_VALUES: usize = 256;
const BITS: usize = 128;
const BLOCKS: usize = 37;
const SCHEDULE_WIDTH: usize = BITS + BLOCKS;
const ACTIVE_ROWS: usize = BITS * BLOCKS;
const H: usize = 0;
const AAD: usize = 128;
const RESULT: usize = 640;
const Y: usize = 768;
const X: usize = 896;
const V: usize = 1024;
const SELECTED: usize = 1152;
const Z: usize = 1280;
const DATA: usize = 1408;
const INPUT: usize = 1536;

#[derive(Clone, Copy)]
pub struct GhashChainAir;

impl BaseAir<Value> for GhashChainAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        SCHEDULE_WIDTH
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for GhashChainAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_ranges(builder);
        self.eval_boundaries(builder);
        self.eval_selection(builder);
        self.eval_data(builder);
        self.eval_transitions(builder);
    }
}
