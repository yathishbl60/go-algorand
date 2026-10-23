// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private canonical ML-KEM polynomial packing and message decoding.
mod binding;
mod layout;
mod operation;
mod schedule;
mod trace;
mod view;
mod write;

use crate::Value;
pub use layout::Wires;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 512;
pub const SCHEDULE_WIDTH: usize = 256;
const COEFFICIENTS: usize = 0;
const PACKED: usize = 256;
const SCRATCH_WIDTH: usize = 63;
const LOCAL_BITS: usize = 25;
const ROLLOVER: usize = 37;
const REMAINDER: usize = 38;

#[derive(Clone, Copy)]
pub struct MlKemEncodingAir {
    bits: usize,
}

impl MlKemEncodingAir {
    /// D=1 decodes message bits to 0/1665; it does not compress coefficients.
    pub const fn new(bits: usize) -> Option<Self> {
        if matches!(bits, 1 | 4 | 10 | 12) {
            Some(Self { bits })
        } else {
            None
        }
    }

    pub const fn bits_per_coefficient(&self) -> usize {
        self.bits
    }

    pub const fn trace_width(&self) -> usize {
        PACKED + 256 * self.bits + SCRATCH_WIDTH
    }

    pub const fn encoded_bytes(&self) -> usize {
        32 * self.bits
    }

    pub(super) const fn scratch(&self) -> usize {
        PACKED + 256 * self.bits
    }
}

impl BaseAir<Value> for MlKemEncodingAir {
    fn width(&self) -> usize {
        self.trace_width()
    }
    fn preprocessed_width(&self) -> usize {
        SCHEDULE_WIDTH
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemEncodingAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_constants(builder);
        self.eval_selection(builder);
        self.eval_operation(builder);
    }
}
