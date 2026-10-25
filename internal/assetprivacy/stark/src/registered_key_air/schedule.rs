// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, PREPROCESSED, ROWS, RegisteredKeyAir};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn build(air: &RegisteredKeyAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    write(air, &mut values, (PREPROCESSED, 0, 0)).expect("registered key schedule");
    RowMajorMatrix::new(values, PREPROCESSED)
}

pub(super) fn write(
    air: &RegisteredKeyAir,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    super::writer::check(values.len(), shape, PREPROCESSED, air.frame_rows())?;
    clear(values, shape, air.frame_rows());
    populate(air, values, shape);
    Some(())
}

fn clear(values: &mut [Value], shape: (usize, usize, usize), rows: usize) {
    for row in values.chunks_exact_mut(shape.0).skip(shape.2).take(rows) {
        row[shape.1..shape.1 + PREPROCESSED].fill(Value::ZERO);
    }
}

fn populate(air: &RegisteredKeyAir, values: &mut [Value], shape: (usize, usize, usize)) {
    if let Some(table) = air.record.preprocessing() {
        copy(values, shape, 0, &table);
    }
    let member = air.membership.frame_preprocessed_trace();
    let target = (shape.0, shape.1, shape.2 + air.record.rows());
    copy(values, target, 1, &member);
    flags(values, shape, 0, air.record.rows());
}

fn copy(
    values: &mut [Value],
    shape: (usize, usize, usize),
    stage: usize,
    table: &RowMajorMatrix<Value>,
) {
    let (stride, start, first) = shape;
    assert!(table.width() <= FLAGS, "key schedule width");
    for local in 0..table.height() {
        let row = &mut values[(first + local) * stride + start..][..PREPROCESSED];
        row[..table.width()]
            .copy_from_slice(&table.values[local * table.width()..][..table.width()]);
    }
    flags(values, shape, stage, table.height());
}

fn flags(values: &mut [Value], shape: (usize, usize, usize), stage: usize, rows: usize) {
    for local in 0..rows {
        let at = (shape.2 + local) * shape.0 + shape.1 + FLAGS + stage * 4;
        values[at] = Value::ONE;
        values[at + 1] = Value::from_bool(local == 0);
        values[at + 2] = Value::from_bool(local + 1 == rows);
        values[at + 3] = Value::from_bool(local + 1 < rows);
    }
}
