// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, FLAGS, FRAME_ROWS, MembershipVerticalAir, PREPROCESSED, STAGE_ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build(rows: usize) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(rows * PREPROCESSED);
    for (index, row) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
        .take(ACTIVE_ROWS)
    {
        let local = index % STAGE_ROWS;
        row[index / STAGE_ROWS] = Value::ONE;
        row[FLAGS] = Value::ONE;
        row[FLAGS + 1] = Value::from_bool(local == 0);
        row[FLAGS + 2] = Value::from_bool(local + 1 == STAGE_ROWS);
        row[FLAGS + 3] = Value::from_bool(local + 1 < STAGE_ROWS);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

impl MembershipVerticalAir {
    pub(crate) fn frame_preprocessed_trace(&self) -> RowMajorMatrix<Value> {
        build(FRAME_ROWS)
    }
}
