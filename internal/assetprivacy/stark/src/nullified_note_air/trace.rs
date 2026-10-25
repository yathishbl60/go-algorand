// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use sha3::{Digest, Sha3_512};

use super::NullifiedNoteAir;
use crate::{Value, sha3_air::digest_values};

impl NullifiedNoteAir {
    /// Builds private columns; the native ledger must separately check freshness.
    pub fn trace(&self, opening: &[u8; 512], secret: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let owned = self.owned.trace(opening, secret)?;
        let message = self.message(opening)?;
        let hash = self.hash.trace(&message)?;
        let digest = digest_values(Sha3_512::digest(message).into());
        Some(self.join(owned, hash, digest))
    }

    /// Computes the public marker; this alone does not validate or authorize a note.
    pub fn nullifier(&self, opening: &[u8; 512]) -> Option<[u8; 64]> {
        Some(Sha3_512::digest(self.message(opening)?).into())
    }

    fn message(&self, opening: &[u8; 512]) -> Option<[u8; 245]> {
        let commitment = self.owned.note_commitment(opening)?;
        Some(crate::nullifier_v2::message(
            self.prefix,
            opening,
            commitment,
        ))
    }

    fn join(
        &self,
        owned: RowMajorMatrix<Value>,
        hash: RowMajorMatrix<Value>,
        digest: [Value; 32],
    ) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(owned.height() * self.width());
        let (nw, hw) = (self.owned.width(), self.hash.width());
        for (i, row) in values.chunks_exact_mut(self.width()).enumerate() {
            row[..nw].copy_from_slice(&owned.values[i * nw..(i + 1) * nw]);
            row[nw..nw + hw].copy_from_slice(&hash.values[i * hw..(i + 1) * hw]);
            row[nw + hw..].copy_from_slice(&digest);
        }
        RowMajorMatrix::new(values, self.width())
    }
}
