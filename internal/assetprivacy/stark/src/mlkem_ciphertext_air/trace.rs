// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemCiphertextAir, PUBLIC_VALUES, ROWS, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

/// Private trace columns; only public_values and ciphertext may be published.
pub struct CiphertextWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
    pub ciphertext: [u8; 1088],
}

impl MlKemCiphertextAir {
    pub fn public_values(&self, ciphertext: [u8; 1088]) -> [Value; PUBLIC_VALUES] {
        core::array::from_fn(|i| {
            Value::from_u16(u16::from_le_bytes([
                ciphertext[2 * i],
                ciphertext[2 * i + 1],
            ]))
        })
    }

    /// Builds witnesses without running constraints, proof generation or verification.
    pub fn trace(&self, u: [[u16; 256]; 3], v: [u16; 256]) -> Option<CiphertextWitness> {
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let ciphertext = self.write_into(&u, &v, &mut values, (WIDTH, 0, 0))?;
        Some(CiphertextWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(ciphertext),
            ciphertext,
        })
    }
}
