// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncodingAir, PACKED, ROWS, write};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemEncodingAir {
    /// Witness generation only. D=1 requires every coefficient to be 0 or 1665.
    pub fn trace(&self, input: [u16; 256]) -> Option<RowMajorMatrix<Value>> {
        if !self.valid_input(&input) {
            return None;
        }
        let width = self.trace_width();
        let mut values = Value::zero_vec(ROWS * width);
        for (index, row) in values.chunks_exact_mut(width).enumerate() {
            self.write_fixed(row, &input);
            let coefficient = input.get(index).copied().unwrap_or(0);
            write::arithmetic(&mut row[self.scratch()..], self.bits, coefficient);
        }
        Some(RowMajorMatrix::new(values, width))
    }

    fn write_fixed(&self, row: &mut [Value], input: &[u16; 256]) {
        for (index, &coefficient) in input.iter().enumerate() {
            row[index] = Value::from_u16(coefficient);
            let encoded = write::encoded(coefficient, self.bits);
            for bit in 0..self.bits {
                row[PACKED + index * self.bits + bit] = Value::from_bool(encoded & (1 << bit) != 0);
            }
        }
    }
}
