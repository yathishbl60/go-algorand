// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemKeygenSeedAir, ROWS, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemKeygenSeedAir {
    pub fn trace(&self, seed: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_into(seed, &mut values, (WIDTH, 0, 0))?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}
