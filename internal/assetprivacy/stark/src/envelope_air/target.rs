// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn check(
    len: usize,
    shape: (usize, usize, usize),
    width: usize,
    rows: usize,
) -> Option<()> {
    let (stride, column, first) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || column.checked_add(width)? > stride {
        return None;
    }
    (first.checked_add(rows)? <= len / stride).then_some(())
}

pub(super) fn clear(values: &mut [Value], shape: (usize, usize, usize), width: usize, rows: usize) {
    let (stride, column, first) = shape;
    for row in values.chunks_exact_mut(stride).skip(first).take(rows) {
        row[column..column + width].fill(Value::ZERO);
    }
}

pub(super) fn shift(
    shape: (usize, usize, usize),
    column: usize,
    row: usize,
) -> (usize, usize, usize) {
    (shape.0, shape.1 + column, shape.2 + row)
}
