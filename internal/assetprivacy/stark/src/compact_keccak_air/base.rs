// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, PREPROCESSED, WIDTH};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

impl<F: PrimeField64> BaseAir<F> for CompactKeccakAir<F> {
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
        (0..WIDTH).collect()
    }
    fn preprocessed_next_row_columns(&self) -> Vec<usize> {
        Vec::new()
    }
    fn max_constraint_degree(&self) -> Option<usize> {
        Some(5)
    }
}

impl<F: PrimeField64, AB: AirBuilder<F = F>> Air<AB> for CompactKeccakAir<F> {
    fn eval(&self, builder: &mut AB) {
        self.eval_bits(builder);
        self.eval_parity(builder);
        self.eval_round(builder);
    }
}
