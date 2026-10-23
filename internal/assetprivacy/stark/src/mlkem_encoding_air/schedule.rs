// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncodingAir, ROWS, SCHEDULE_WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemEncodingAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * SCHEDULE_WIDTH);
        for index in 0..256 {
            values[index * SCHEDULE_WIDTH + index] = Value::ONE;
        }
        RowMajorMatrix::new(values, SCHEDULE_WIDTH)
    }
}
