// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{AesGcmAir, ROWS, SCHEDULE};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl AesGcmAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * SCHEDULE);
        for row in 0..13 { values[row * SCHEDULE + row] = Value::ONE; }
        for step in 0..476 {
            let row = step + 13;
            values[row * SCHEDULE + 13 + step % 14] = Value::ONE;
            values[row * SCHEDULE + 27 + step / 14] = Value::ONE;
        }
        RowMajorMatrix::new(values, SCHEDULE)
    }
}

pub(super) fn rounds<AB: AirBuilder<F = Value>>(builder: &AB, end: usize) -> AB::Expr {
    builder.preprocessed().current_slice()[13..13 + end].iter().map(|&v| v.into()).sum()
}
