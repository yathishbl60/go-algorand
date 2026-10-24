// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CT_BLOCK, DIGEST, END, FRAME, KEY_BLOCK, PREPROCESSED, ROWS};
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
        write(row, index);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn write(row: &mut [Value], index: usize) {
    let block = index / 24;
    if index < 864 {
        row[FRAME] = Value::ONE;
        row[FRAME + 1] = Value::from_bool(index.is_multiple_of(24));
        row[FRAME + 2] = Value::from_bool(index % 24 == 23);
        row[FRAME + 3] = Value::from_bool(index % 24 < 23);
        if (3..20).contains(&block) {
            row[KEY_BLOCK + block - 3] = Value::ONE;
        } else if block >= 20 {
            row[CT_BLOCK + block - 20] = Value::ONE;
        }
    }
    if index < 840 && index % 24 == 23 {
        row[END + block] = Value::ONE;
    }
    row[DIGEST] = Value::from_bool(index == 863);
}
