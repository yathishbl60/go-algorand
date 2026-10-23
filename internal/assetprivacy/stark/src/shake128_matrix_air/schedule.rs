// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Shake128MatrixAir;
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_ROUNDS;
use p3_matrix::dense::RowMajorMatrix;

impl Shake128MatrixAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(self.rows * self.blocks);
        for block in 0..self.blocks {
            values[((block + 1) * NUM_ROUNDS - 1) * self.blocks + block] = Value::ONE;
        }
        RowMajorMatrix::new(values, self.blocks)
    }
}
