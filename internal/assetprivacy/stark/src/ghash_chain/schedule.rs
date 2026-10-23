// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, BITS, GhashChainAir, ROWS, SCHEDULE_WIDTH};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl GhashChainAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * SCHEDULE_WIDTH);
        for round in 0..ACTIVE_ROWS {
            values[round * SCHEDULE_WIDTH + round % BITS] = Value::ONE;
            values[round * SCHEDULE_WIDTH + BITS + round / BITS] = Value::ONE;
        }
        RowMajorMatrix::new(values, SCHEDULE_WIDTH)
    }
}

pub(super) fn active<AB: AirBuilder<F = Value>>(builder: &AB) -> AB::Expr {
    builder.preprocessed().current_slice()[..BITS]
        .iter()
        .map(|&v| v.into())
        .sum()
}
