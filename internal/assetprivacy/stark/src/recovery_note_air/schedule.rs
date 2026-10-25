// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    FLAGS, FRAME_ROWS, MEMBER_ROW, NOTE_ROWS, NULLIFIER_ROW, NULLIFIER_ROWS, PREPROCESSED, ROWS,
    RecoveryNoteAir,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn build(air: &RecoveryNoteAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    write(air, &mut values, (PREPROCESSED, 0, 0)).expect("recovery note schedule");
    RowMajorMatrix::new(values, PREPROCESSED)
}

pub(super) fn write(
    air: &RecoveryNoteAir,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    super::writer::check(values.len(), shape, PREPROCESSED)?;
    clear(values, shape);
    copy_member(air, values, shape);
    for (stage, first, rows) in [
        (0, 0, NOTE_ROWS),
        (1, NULLIFIER_ROW, NULLIFIER_ROWS),
        (2, MEMBER_ROW, crate::membership_vertical::FRAME_ROWS),
    ] {
        flags(values, (shape.0, shape.1, shape.2 + first), stage, rows);
    }
    Some(())
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

fn clear(values: &mut [Value], shape: (usize, usize, usize)) {
    for row in values
        .chunks_exact_mut(shape.0)
        .skip(shape.2)
        .take(FRAME_ROWS)
    {
        row[shape.1..shape.1 + PREPROCESSED].fill(Value::ZERO);
    }
}

fn copy_member(air: &RecoveryNoteAir, values: &mut [Value], shape: (usize, usize, usize)) {
    let member = air.membership.frame_preprocessed_trace();
    for local in 0..member.height() {
        let start = (shape.2 + MEMBER_ROW + local) * shape.0 + shape.1;
        values[start..start + FLAGS].copy_from_slice(&member.values[local * FLAGS..][..FLAGS]);
    }
}
