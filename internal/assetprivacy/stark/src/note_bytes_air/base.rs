// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BIT_COUNT, NoteBytesAir, PREPROCESSED, PREVIOUS, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

impl<F: PrimeField64> BaseAir<F> for NoteBytesAir<F> {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<F>> {
        Some(self.program())
    }
    fn main_next_row_columns(&self) -> Vec<usize> {
        vec![PREVIOUS, BIT_COUNT]
    }
    fn preprocessed_next_row_columns(&self) -> Vec<usize> {
        Vec::new()
    }
    fn max_constraint_degree(&self) -> Option<usize> {
        Some(3)
    }
}

impl<F: PrimeField64, AB: AirBuilder<F = F>> Air<AB> for NoteBytesAir<F> {
    fn eval(&self, builder: &mut AB) {
        self.eval_bytes(builder);
        self.eval_public(builder);
        self.eval_continuity(builder);
        self.eval_nonzero(builder);
    }
}
