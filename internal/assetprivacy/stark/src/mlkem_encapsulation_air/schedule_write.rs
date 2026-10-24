// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, PREPROCESSED};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn copy_ref(
    values: &mut [Value],
    shape: (usize, usize, usize),
    first: usize,
    stage: usize,
    table: &RowMajorMatrix<Value>,
) {
    assert!(table.width() <= CHILD_PRE, "fixed child preprocessing");
    let (stride, column, base) = shape;
    for local in 0..table.height() {
        let row = &mut values[(base + first + local) * stride + column..][..PREPROCESSED];
        row[..table.width()]
            .copy_from_slice(&table.values[local * table.width()..][..table.width()]);
    }
    flags(values, shape, first, stage, table.height());
}

pub(super) fn flags(
    values: &mut [Value],
    shape: (usize, usize, usize),
    first: usize,
    stage: usize,
    height: usize,
) {
    let (stride, column, base) = shape;
    for local in 0..height {
        let row = &mut values[(base + first + local) * stride + column..][..PREPROCESSED];
        let start = CHILD_PRE + 4 * stage;
        row[start] = Value::ONE;
        row[start + 1] = Value::from_bool(local == 0);
        row[start + 2] = Value::from_bool(local + 1 == height);
        row[start + 3] = Value::from_bool(local + 1 < height);
    }
}
