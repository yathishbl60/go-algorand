// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, ROWS};
use crate::{Value, serial_keccak::RC_POSITIONS};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::RC;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for (index, row) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
    {
        row[3] = Value::from_bool(index == 72);
        if let Some((phase, round)) = stage(index) {
            row[phase] = Value::ONE;
            for (i, bit) in RC_POSITIONS.iter().enumerate() {
                row[4 + i] = Value::from_bool(phase == 2 && RC[round] & (1 << bit) != 0);
            }
        }
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

pub(super) fn stage(row: usize) -> Option<(usize, usize)> {
    let step = match row {
        0..72 => row,
        72 => return None,
        73..145 => row - 73,
        _ => return None,
    };
    Some((step % 3, step / 3))
}
