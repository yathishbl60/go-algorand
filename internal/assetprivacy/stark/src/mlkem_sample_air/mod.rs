// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private first-accepted ML-KEM SampleNTT prefix relation, without SHAKE binding.

mod binding;
mod cursor;
mod layout;
mod schedule;
mod trace;
mod witness;
mod writer;

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

#[derive(Clone, Copy)]
pub struct MlKemSampleAir {
    rate_blocks: usize,
}

impl MlKemSampleAir {
    /// Select a trusted SHAKE128 prefix size; exhaustion returns no witness.
    pub fn new(rate_blocks: usize) -> Option<Self> {
        if !(1..=crate::shake128_matrix_air::MAX_RATE_BLOCKS).contains(&rate_blocks) {
            return None;
        }
        Some(Self { rate_blocks })
    }
}

impl BaseAir<Value> for MlKemSampleAir {
    fn width(&self) -> usize {
        self.prefix_bytes() * 8 + 539
    }
    fn preprocessed_width(&self) -> usize {
        self.candidate_count()
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemSampleAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_fixed(builder);
        self.eval_candidate(builder);
        self.eval_comparison(builder);
        self.eval_cursor(builder);
        self.eval_output(builder);
    }
}
