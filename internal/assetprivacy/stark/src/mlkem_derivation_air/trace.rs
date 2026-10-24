// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    KEY_BYTES, MlKemDerivationAir, ROWS,
    witness::{Inputs, coefficient},
};
use crate::{
    Value,
    sha3_air::{digest_values, digest256_values},
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemDerivationAir {
    /// Private canonical-key/G witness only; no K-PKE or acceptance check runs.
    pub fn trace(
        &self,
        key: [u8; KEY_BYTES],
        randomness: [u8; 32],
    ) -> Option<RowMajorMatrix<Value>> {
        let input = Inputs::new(key, randomness)?;
        let mut values = Value::zero_vec(ROWS * self.width());
        self.key_hash
            .write_into(&input.key, &mut values, self.width(), 0)?;
        self.expand.write_into(
            &input.message,
            &mut values,
            self.width(),
            self.key_hash.width(),
        )?;
        for (index, row) in values.chunks_exact_mut(self.width()).enumerate() {
            self.write_auxiliary(row, &input, index);
        }
        Some(RowMajorMatrix::new(values, self.width()))
    }

    fn write_auxiliary(&self, row: &mut [Value], input: &Inputs, index: usize) {
        let start = self.digest_start();
        row[start..start + 16].copy_from_slice(&digest256_values(input.digest));
        row[start + 16..start + 48].copy_from_slice(&digest_values(input.output));
        for (bit, value) in row[self.output_start()..self.scratch()]
            .iter_mut()
            .enumerate()
        {
            *value = Value::from_bool(input.output[bit / 8] & (1 << (bit % 8)) != 0);
        }
        let value = coefficient(&input.key, index);
        for bit in 0..12 {
            row[self.scratch() + bit] = Value::from_bool(value & (1 << bit) != 0);
            row[self.scratch() + 12 + bit] = Value::from_bool((3328 - value) & (1 << bit) != 0);
        }
    }
}
