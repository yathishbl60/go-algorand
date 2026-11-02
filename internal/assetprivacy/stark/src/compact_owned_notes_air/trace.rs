// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HASH, INVERSE, NOTE, OwnedNotesAir, RELAY, RELAY_COUNT, WIDTH};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

impl<F: PrimeField64> OwnedNotesAir<F> {
    /// Validates canonical openings and exact SHA3 ownership before trace allocation.
    pub fn trace(&self, notes: &[[u8; 512]], secrets: &[[u8; 32]]) -> Option<RowMajorMatrix<F>> {
        if !self.valid_witness(notes, secrets) {
            return None;
        }
        let note = self.note.trace(notes)?;
        let inputs: Vec<_> = secrets
            .iter()
            .map(|secret| self.owner_input(secret))
            .collect();
        let hash = self.hash.trace(&inputs)?;
        let mut values = self.join(note, hash, notes)?;
        self.write_inverses(&mut values, secrets)?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }

    fn join(
        &self,
        note: RowMajorMatrix<F>,
        hash: RowMajorMatrix<F>,
        notes: &[[u8; 512]],
    ) -> Option<Vec<F>> {
        let mut values = Vec::new();
        values.try_reserve_exact(self.rows * WIDTH).ok()?;
        values.resize(self.rows * WIDTH, F::ZERO);
        for (index, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            row[NOTE..HASH].copy_from_slice(&note.values[index * 11..(index + 1) * 11]);
            row[HASH..HASH + 1920].copy_from_slice(&hash.values[index * 1920..(index + 1) * 1920]);
            self.write_owner(row, index, notes);
            for column in 0..RELAY_COUNT {
                row[RELAY + column] = self.periodic[column][index];
            }
        }
        Some(values)
    }

    fn write_inverses(&self, values: &mut [F], secrets: &[[u8; 32]]) -> Option<()> {
        for (index, secret) in secrets.iter().enumerate() {
            let count: u32 = secret.iter().map(|byte| byte.count_ones()).sum();
            values[index * 24 * WIDTH + INVERSE] = F::from_u32(count).try_inverse()?;
        }
        Some(())
    }
}
