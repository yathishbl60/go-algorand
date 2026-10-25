// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, INPUT_ROWS, OUTPUT_ROW, OUTPUT_ROWS, PREPROCESSED, ROWS, SpendGroupAir};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build(air: &SpendGroupAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for index in 0..2 {
        air.input
            .write_preprocessed_into(&mut values, (PREPROCESSED, 0, index * INPUT_ROWS))
            .expect("spendable schedule");
        flags(&mut values, index, index * INPUT_ROWS, INPUT_ROWS);
    }
    for index in 0..air.operation.output_count() {
        flags(
            &mut values,
            index + 2,
            OUTPUT_ROW + index * OUTPUT_ROWS,
            OUTPUT_ROWS,
        );
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn flags(values: &mut [Value], stage: usize, first: usize, rows: usize) {
    for local in 0..rows {
        let start = (first + local) * PREPROCESSED + FLAGS + 4 * stage;
        values[start] = Value::ONE;
        values[start + 1] = Value::from_bool(local == 0);
        values[start + 2] = Value::from_bool(local + 1 == rows);
        values[start + 3] = Value::from_bool(local + 1 < rows);
    }
}
