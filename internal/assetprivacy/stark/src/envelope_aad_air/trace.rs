// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EnvelopeAadAir, PUBLIC_VALUES, ROWS, WIDTH, witness::Inputs};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl EnvelopeAadAir {
    pub fn trace(
        &self,
        key: &[u8],
        epoch: u64,
        ciphertext: [u8; 1088],
    ) -> Option<RowMajorMatrix<Value>> {
        let input = Inputs::new(&self.context, key, epoch, ciphertext)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_rows(&mut values, (WIDTH, 0, 0), &input);
        Some(RowMajorMatrix::new(values, WIDTH))
    }
    pub fn public_values(&self, ciphertext: [u8; 1088]) -> [Value; PUBLIC_VALUES] {
        core::array::from_fn(|i| {
            Value::from_u16(u16::from_le_bytes([
                ciphertext[2 * i],
                ciphertext[2 * i + 1],
            ]))
        })
    }
}
