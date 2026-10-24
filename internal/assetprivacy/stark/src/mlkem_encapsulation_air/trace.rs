// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncapsulationAir, PUBLIC_VALUES, ROWS, WIDTH, fixed, writer};
use crate::{
    Value,
    mlkem_dot_air::{DotInputs, MlKemDotAir},
    mlkem_host::{EncryptionWitness, encapsulate},
    mlkem_pke_air::MlKemPkeAir,
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

/// Secret witness columns. Only the 1088-byte KEM ciphertext is public.
pub struct EncapsulationWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
}

impl MlKemEncapsulationAir {
    pub fn public_values(&self, ciphertext: [u8; 1088]) -> [Value; PUBLIC_VALUES] {
        MlKemPkeAir.public_values(ciphertext)
    }

    /// Caller supplies fresh secret uniform m. No randomness or proof is generated.
    pub fn trace(&self, public_key: &[u8], message: [u8; 32]) -> Option<EncapsulationWitness> {
        let input = encapsulate(public_key, message)?;
        let dots = dots(&input.encryption)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        writer::write(public_key, message, &input, &mut values)?;
        fixed::write(&mut values, &input, &dots);
        Some(EncapsulationWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(input.encryption.ciphertext),
        })
    }
}

fn dots(input: &EncryptionWitness) -> Option<[[u16; 256]; 4]> {
    MlKemDotAir.values(&DotInputs {
        matrix: input.matrix,
        public_polynomials: input.public_polynomials,
        r_hat: input.r_hat,
    })
}
