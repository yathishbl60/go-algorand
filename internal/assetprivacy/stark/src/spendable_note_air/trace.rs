// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PUBLIC_VALUES, ROWS, SpendInput, SpendableNoteAir, WIDTH, fixed, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct SpendNoteWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
}

impl SpendableNoteAir {
    pub fn public_values(
        &self,
        nullifier: [u8; 64],
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
    ) -> [Value; PUBLIC_VALUES] {
        let parts = [
            digest_values(nullifier),
            digest_values(note_root),
            digest_values(eligibility_root),
        ];
        core::array::from_fn(|i| parts[i / 32][i % 32])
    }
    pub(crate) fn check_inputs(
        &self,
        input: SpendInput<'_>,
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
    ) -> Option<[u8; 64]> {
        Some(Prepared::new(self, input, note_root, eligibility_root)?.nullifier)
    }
    pub fn trace(
        &self,
        input: SpendInput<'_>,
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
    ) -> Option<SpendNoteWitness> {
        let prepared = Prepared::new(self, input, note_root, eligibility_root)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_prepared(&prepared, eligibility_root, &mut values, (WIDTH, 0, 0))?;
        fixed::write(&mut values, &prepared, (WIDTH, 0, 0), ROWS);
        Some(SpendNoteWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(prepared.nullifier, note_root, eligibility_root),
        })
    }
}
