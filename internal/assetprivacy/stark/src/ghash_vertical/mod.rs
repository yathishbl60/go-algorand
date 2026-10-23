// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private vertical GHASH multiplication with a trusted fixed one-hot schedule.
//! This is not a complete encryption relation or an accepted native verifier.

mod binding;
mod step;
mod trace;
mod wires;

pub use wires::{Wires, wires};

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 512;
pub const WIDTH: usize = 769;
const BITS: usize = 128;
const LEFT: usize = 0;
const RIGHT: usize = 128;
const POWER: usize = 256;
const SELECTED: usize = 384;
const ACCUMULATOR: usize = 512;
const OUTPUT: usize = 640;
const INPUT: usize = 768;

#[derive(Clone, Copy)]
pub struct GhashProductAir;

impl BaseAir<Value> for GhashProductAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        BITS
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for GhashProductAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_ranges(builder);
        self.eval_boundaries(builder);
        self.eval_selection(builder);
        self.eval_steps(builder);
    }
}
