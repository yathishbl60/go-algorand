// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CODEC_ROW, DOT_ROW, FINAL, INV_SPAN, INVERSE_ROW, MlKemPkeAir, PUBLIC_VALUES, ROWS, WIDTH,
    WORK, prepared::Prepared, write_add, write_fixed,
};
use crate::{
    Value, mlkem_ciphertext_air::MlKemCiphertextAir, mlkem_dot_air::MlKemDotAir,
    mlkem_host::EncryptionWitness, mlkem_ntt_air::MlKemNttAir,
};
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
        transforms(&prepared, &mut values)?;
        MlKemDotAir.write_into(&prepared.dot_input, &mut values, (WIDTH, 0, DOT_ROW))?;
        write_add::write(&mut values, &prepared);
        let ciphertext = MlKemCiphertextAir.write_into(
            &input.u,
            &input.v,
            &mut values,
            (WIDTH, FINAL, CODEC_ROW),
        )?;
        write_fixed::write(&mut values, &prepared);
        Some(PkeWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(ciphertext),
        })
    }
}

fn transforms(input: &Prepared<'_>, values: &mut [Value]) -> Option<()> {
    for slot in 0..3 {
        MlKemNttAir::forward().write_into(
            input.input.noise[slot],
            values,
            (WIDTH, WORK, slot * 1024),
        )?;
    }
    for slot in 0..4 {
        let first = INVERSE_ROW + slot * INV_SPAN;
        MlKemNttAir::inverse().write_into(input.dots[slot], values, (WIDTH, WORK, first))?;
    }
    Some(())
}
