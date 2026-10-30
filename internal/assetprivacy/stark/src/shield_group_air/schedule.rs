// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, ROWS};
use crate::{MIN_TRACE_ROWS, Value};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for stage in 0..2 {
        for local in 0..MIN_TRACE_ROWS {
            let start = (stage * MIN_TRACE_ROWS + local) * PREPROCESSED + stage * 4;
            values[start] = Value::ONE;
            values[start + 1] = Value::from_bool(local == 0);
            values[start + 2] = Value::from_bool(local + 1 == MIN_TRACE_ROWS);
            values[start + 3] = Value::from_bool(local + 1 < MIN_TRACE_ROWS);
        }
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}
