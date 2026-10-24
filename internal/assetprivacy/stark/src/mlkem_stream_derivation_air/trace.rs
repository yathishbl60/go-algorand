// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemStreamDerivationAir, ROWS, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemStreamDerivationAir {
    /// Private canonical-key/G witness only; no constraints or proof run.
    pub fn trace(&self, key: &[u8], message: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let input = super::witness::Inputs::new(key, message)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        super::writer::write_rows(&mut values, (WIDTH, 0, 0), &input);
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}
