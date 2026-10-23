// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MemberNoteAir;
use crate::{Value, membership_air::MembershipPath};
use p3_field::{PrimeCharacteristicRing, PrimeField64};
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl MemberNoteAir {
    /// Private witness generation; the ledger must check accepted root and fresh nullifier.
    pub fn trace(
        &self,
        opening: &[u8; 512],
        secret: [u8; 32],
        path: &MembershipPath,
    ) -> Option<RowMajorMatrix<Value>> {
        let note = self.note.trace(opening, secret)?;
        let limbs = self
            .note
            .commitment_limbs(&note.values[..self.note.width()])?;
        let leaf = core::array::from_fn(|i| {
            let limb = limbs[i / 2].as_canonical_u64() as u16;
            limb.to_le_bytes()[i % 2]
        });
        let membership = self.membership.trace(leaf, path)?;
        Some(self.join(note, membership))
    }

    fn join(
        &self,
        note: RowMajorMatrix<Value>,
        membership: RowMajorMatrix<Value>,
    ) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(note.height() * self.width());
        let (nw, mw) = (self.note.width(), self.membership.width());
        for (r, row) in values.chunks_exact_mut(self.width()).enumerate() {
            row[..nw].copy_from_slice(&note.values[r * nw..(r + 1) * nw]);
            row[nw..].copy_from_slice(&membership.values[r * mw..(r + 1) * mw]);
        }
        RowMajorMatrix::new(values, self.width())
    }
}
