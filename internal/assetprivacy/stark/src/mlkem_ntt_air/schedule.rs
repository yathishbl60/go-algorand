// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNttAir, SCALE, SCHEDULE_WIDTH, ZETA};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemNttAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(self.trace_rows() * SCHEDULE_WIDTH);
        for (step, row) in values
            .as_chunks_mut::<SCHEDULE_WIDTH>()
            .0
            .iter_mut()
            .enumerate()
        {
            if let Some(op) = self.operation(step) {
                row[op.left] = Value::ONE;
                if let Some(right) = op.right {
                    row[256 + right] = Value::ONE;
                }
                row[ZETA] = Value::from_u16(op.zeta);
                row[SCALE] = Value::from_bool(op.scale);
            }
        }
        RowMajorMatrix::new(values, SCHEDULE_WIDTH)
    }
}
