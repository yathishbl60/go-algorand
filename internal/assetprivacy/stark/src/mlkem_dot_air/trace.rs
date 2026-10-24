// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DotInputs, MlKemDotAir, Polynomial, ROWS, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemDotAir {
    /// Computes canonical dot products without allocating a field trace matrix.
    pub fn values(&self, input: &DotInputs) -> Option<[Polynomial; 4]> {
        if !input.valid() {
            return None;
        }
        let mut output = [[0; 256]; 4];
        for (index, polynomial) in output.iter_mut().enumerate() {
            *polynomial = input.dot(index)?;
        }
        Some(output)
    }
    /// Private witness construction only; no constraints or proof are evaluated.
    pub fn trace(&self, input: &DotInputs) -> Option<RowMajorMatrix<Value>> {
        if !input.valid() {
            return None;
        }
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_into(input, &mut values, (WIDTH, 0, 0))?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}
