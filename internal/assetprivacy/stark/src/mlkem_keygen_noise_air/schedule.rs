// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for stage in 0..7 {
        for local in 0..512 {
            let start = (stage * 512 + local) * PREPROCESSED + stage * 4;
            values[start] = Value::ONE;
            values[start + 1] = Value::from_bool(local == 0);
            values[start + 2] = Value::from_bool(local == 511);
            values[start + 3] = Value::from_bool(local < 511);
        }
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}
