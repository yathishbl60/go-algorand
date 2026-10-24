// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemDerivationAir, ROWS};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemDerivationAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * COEFFICIENTS);
        for index in 0..COEFFICIENTS {
            values[index * COEFFICIENTS + index] = Value::ONE;
        }
        RowMajorMatrix::new(values, COEFFICIENTS)
    }
}
