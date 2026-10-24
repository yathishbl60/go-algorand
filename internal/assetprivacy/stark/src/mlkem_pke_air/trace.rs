// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemPkeAir, PUBLIC_VALUES, ROWS, WIDTH, prepared::Prepared};
use crate::{Value, mlkem_ciphertext_air::MlKemCiphertextAir, mlkem_host::EncryptionWitness};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

/// Secret proof columns with only the ciphertext exposed as public values.
pub struct PkeWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
}

impl MlKemPkeAir {
    pub fn public_values(&self, ciphertext: [u8; 1088]) -> [Value; PUBLIC_VALUES] {
        MlKemCiphertextAir.public_values(ciphertext)
    }

    /// Constructs checked host witness columns only; no constraints or proof run.
    pub fn trace(&self, input: &EncryptionWitness) -> Option<PkeWitness> {
        let prepared = Prepared::new(input)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let ciphertext = self.write_prepared(&prepared, &mut values, (WIDTH, 0, 0))?;
        Some(PkeWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(ciphertext),
        })
    }
}
