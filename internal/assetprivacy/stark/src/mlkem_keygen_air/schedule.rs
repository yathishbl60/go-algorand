// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE_PRE, CORE_ROWS, DOT_ROWS, FLAGS, MlKemKeygenAir, PREPROCESSED, ROWS};
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemKeygenAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * PREPROCESSED);
        copy(&mut values, &self.core.frame_preprocessed_trace(), 0, 0);
        copy(
            &mut values,
            &self.dot.preprocessed_trace().expect("dot schedule"),
            CORE_ROWS,
            CORE_PRE,
        );
        flags(&mut values, 0, CORE_ROWS, FLAGS);
        flags(&mut values, CORE_ROWS, DOT_ROWS, FLAGS + 4);
        RowMajorMatrix::new(values, PREPROCESSED)
    }
}
fn copy(values: &mut [Value], child: &RowMajorMatrix<Value>, first: usize, column: usize) {
    for (i, row) in child.values.chunks_exact(child.width).enumerate() {
        let start = (first + i) * PREPROCESSED + column;
        values[start..start + child.width].copy_from_slice(row);
    }
}
fn flags(values: &mut [Value], first: usize, rows: usize, column: usize) {
    for local in 0..rows {
        let start = (first + local) * PREPROCESSED + column;
        values[start] = Value::ONE;
        values[start + 1] = Value::from_bool(local == 0);
        values[start + 2] = Value::from_bool(local + 1 == rows);
        values[start + 3] = Value::from_bool(local + 1 < rows);
    }
}
