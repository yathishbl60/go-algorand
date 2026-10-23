// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemSampleAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let candidates = self.candidate_count();
        let mut values = Value::zero_vec(self.trace_rows() * candidates);
        for index in 0..candidates {
            values[index * candidates + index] = Value::ONE;
        }
        RowMajorMatrix::new(values, candidates)
    }

    pub(super) fn active<AB: AirBuilder<F = Value>>(&self, builder: &AB) -> AB::Expr {
        builder
            .preprocessed()
            .current_slice()
            .iter()
            .copied()
            .map(Into::into)
            .sum()
    }
}
