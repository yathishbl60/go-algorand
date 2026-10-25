// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CurrentEligibilityAir, FLAGS, FRAME_ROWS, MEMBER_ROW, PREPROCESSED, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn build(air: &CurrentEligibilityAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    write(air, &mut values, (PREPROCESSED, 0, 0)).expect("eligibility schedule");
    RowMajorMatrix::new(values, PREPROCESSED)
}

pub(super) fn write(
    air: &CurrentEligibilityAir,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    super::writer::check(values.len(), shape, PREPROCESSED, FRAME_ROWS)?;
    clear(values, shape);
    let member = air.membership.frame_preprocessed_trace();
    copy(values, (shape.0, shape.1, shape.2 + MEMBER_ROW), &member);
    flags(values, shape, 0, MEMBER_ROW);
    flags(
        values,
        (shape.0, shape.1, shape.2 + MEMBER_ROW),
        1,
        member.height(),
    );
    Some(())
}

fn clear(values: &mut [Value], shape: (usize, usize, usize)) {
    for row in values
        .chunks_exact_mut(shape.0)
        .skip(shape.2)
        .take(FRAME_ROWS)
    {
        row[shape.1..shape.1 + PREPROCESSED].fill(Value::ZERO);
    }
}

fn copy(values: &mut [Value], shape: (usize, usize, usize), table: &RowMajorMatrix<Value>) {
    assert_eq!(
        table.width(),
        FLAGS,
        "eligibility membership schedule width"
    );
    for local in 0..table.height() {
        let row = &mut values[(shape.2 + local) * shape.0 + shape.1..][..PREPROCESSED];
        row[..FLAGS].copy_from_slice(&table.values[local * FLAGS..][..FLAGS]);
    }
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
