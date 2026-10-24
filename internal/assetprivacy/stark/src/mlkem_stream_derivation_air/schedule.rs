// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK, END, FRAME, G_END, G_FIRST, H_END, PREPROCESSED, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for (index, row) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
    {
        if index < 768 {
            row[index] = Value::ONE;
        }
        if index < 216 {
            row[BLOCK + index / 24] = Value::ONE;
        }
        frames(row, index);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn frames(row: &mut [Value], index: usize) {
    if index < 240 {
        row[FRAME] = Value::ONE;
        row[FRAME + 1] = Value::from_bool(index.is_multiple_of(24));
        row[FRAME + 2] = Value::from_bool(index % 24 == 23);
        row[FRAME + 3] = Value::from_bool(index % 24 < 23);
    }
    if index < 192 && index % 24 == 23 {
        row[END + index / 24] = Value::ONE;
    }
    row[H_END] = Value::from_bool(index == 215);
    row[G_END] = Value::from_bool(index == 239);
    row[G_FIRST] = Value::from_bool(index == 216);
}
