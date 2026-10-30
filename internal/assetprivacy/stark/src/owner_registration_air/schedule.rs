// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for (row, flags) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
    {
        flags[0] = Value::ONE;
        flags[1] = Value::from_bool(row == 0);
        flags[2] = Value::from_bool(row + 1 == ROWS);
        flags[3] = Value::from_bool(row + 1 < ROWS);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}
