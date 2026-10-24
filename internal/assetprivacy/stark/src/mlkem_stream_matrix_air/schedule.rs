// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, BLOCK_ROWS, FRAME, LATER, PREPROCESSED, RATE_CARRY, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn schedule() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    let rows = values.as_chunks_mut::<PREPROCESSED>().0;
    for (index, row) in rows.iter_mut().enumerate() {
        let offset = index % BLOCK_ROWS;
        let active = index < ACTIVE_ROWS;
        if active && offset >= 24 {
            row[offset - 24] = Value::ONE;
        }
        row[FRAME] = Value::from_bool(active && offset < 24);
        row[FRAME + 1] = Value::from_bool(active && offset == 0);
        row[FRAME + 2] = Value::from_bool(active && offset == 23);
        row[FRAME + 3] = Value::from_bool(active && offset < 23);
        row[LATER] = Value::from_bool(active && offset == 0 && index > 0);
        row[RATE_CARRY] = Value::from_bool(!active || offset != 135 || index + 1 == ACTIVE_ROWS);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}
