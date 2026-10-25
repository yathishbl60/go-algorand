// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AuthorizedRecipientAir, CHILD_PRE, PREPROCESSED, ROWS, SPEND_ROW, SPEND_ROWS, VIEW_ROW,
    VIEW_ROWS,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build(air: &AuthorizedRecipientAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    air.envelope
        .write_preprocessed_into(&mut values, (PREPROCESSED, 0, 0))
        .expect("envelope schedule");
    air.view
        .write_preprocessed_into(&mut values, (PREPROCESSED, 0, VIEW_ROW))
        .expect("view schedule");
    air.spend
        .write_preprocessed_into(&mut values, (PREPROCESSED, 0, SPEND_ROW))
        .expect("spend schedule");
    for (stage, first, rows) in [
        (0, 0, VIEW_ROW),
        (1, VIEW_ROW, VIEW_ROWS),
        (2, SPEND_ROW, SPEND_ROWS),
    ] {
        flags(&mut values, stage, first, rows);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn flags(values: &mut [Value], stage: usize, first: usize, rows: usize) {
    for local in 0..rows {
        let start = (first + local) * PREPROCESSED + CHILD_PRE + stage * 4;
        values[start] = Value::ONE;
        values[start + 1] = Value::from_bool(local == 0);
        values[start + 2] = Value::from_bool(local + 1 == rows);
        values[start + 3] = Value::from_bool(local + 1 < rows);
    }
}
