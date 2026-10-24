// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNttAir, WIDTH};
use crate::{Value, mlkem_field::Q};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemNttAir {
    /// Private witness generation only. Noncanonical input coefficients are rejected.
    pub fn trace(&self, input: [u16; 256]) -> Option<RowMajorMatrix<Value>> {
        if input.iter().any(|&value| value >= Q) {
            return None;
        }
        let mut values = Value::zero_vec(self.trace_rows() * WIDTH);
        self.write_into(input, &mut values, (WIDTH, 0, 0))?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }

    /// Computes the same witness arithmetic without allocating a trace matrix.
    pub fn values(&self, input: [u16; 256]) -> Option<[u16; 256]> {
        if input.iter().any(|&value| value >= Q) {
            return None;
        }
        let mut state = input;
        for step in 0..self.trace_rows() {
            let op = self.operation(step);
            let arithmetic = self.arithmetic(&state, op.as_ref());
            super::writer::advance(&mut state, op.as_ref(), &arithmetic);
        }
        Some(state)
    }
}
