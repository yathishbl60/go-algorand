// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use sha3::{Digest, Sha3_512};

use super::OwnedNoteAir;
use crate::{Value, sha3_air::digest_values};

struct TraceParts {
    note: RowMajorMatrix<Value>,
    owner: RowMajorMatrix<Value>,
    digests: [Value; 64],
    inverse: Value,
}

impl OwnedNoteAir {
    /// Builds private ownership columns; neither the opening nor secret is public.
    pub fn trace(&self, opening: &[u8; 512], secret: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let message = self.owner_message(opening, secret)?;
        let count: u32 = secret.iter().map(|v| v.count_ones()).sum();
        let parts = TraceParts {
            note: self.note.trace(opening)?,
            owner: self.owner.trace(&message)?,
            digests: self.digests(opening, &message)?,
            inverse: Value::from_u32(count).inverse(),
        };
        Some(self.join(parts))
    }

    fn owner_message(&self, opening: &[u8; 512], secret: [u8; 32]) -> Option<Vec<u8>> {
        if secret == [0; 32] {
            return None;
        }
        let mut message = self.prefix.clone();
        message.extend(secret);
        let digest: [u8; 64] = Sha3_512::digest(&message).into();
        (digest == opening[73..137]).then_some(message)
    }

    fn digests(&self, opening: &[u8; 512], message: &[u8]) -> Option<[Value; 64]> {
        let mut values = [Value::ZERO; 64];
        values[..32].copy_from_slice(&digest_values(self.note.commitment(opening)?));
        values[32..].copy_from_slice(&digest_values(Sha3_512::digest(message).into()));
        Some(values)
    }

    fn join(&self, parts: TraceParts) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(parts.note.height() * self.width());
        let (nw, ow) = (self.note.width(), self.owner.width());
        for (i, row) in values.chunks_exact_mut(self.width()).enumerate() {
            row[..nw].copy_from_slice(&parts.note.values[i * nw..(i + 1) * nw]);
            row[nw..nw + ow].copy_from_slice(&parts.owner.values[i * ow..(i + 1) * ow]);
            row[nw + ow..nw + ow + 64].copy_from_slice(&parts.digests);
            row[nw + ow + 64] = parts.inverse;
        }
        RowMajorMatrix::new(values, self.width())
    }
}
