// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use sha3::{Digest, Sha3_512};

use super::{CONTENT_SIZE, ENCODED_SIZE, NONZERO, NoteCommitmentAir, OPENING_START};
use crate::Value;

impl NoteCommitmentAir {
    pub(crate) fn commitment(&self, opening: &[u8; ENCODED_SIZE]) -> Option<[u8; 64]> {
        Some(Sha3_512::digest(self.message(opening)?).into())
    }

    /// Builds a private trace from the Go note package's canonical 512-byte opening.
    pub fn trace(&self, opening: &[u8; ENCODED_SIZE]) -> Option<RowMajorMatrix<Value>> {
        self.message(opening)?;
        let mut values = Value::zero_vec(crate::MIN_TRACE_ROWS * self.width());
        self.write_into(opening, &mut values, (self.width(), 0, 0))?;
        Some(RowMajorMatrix::new(values, self.width()))
    }

    pub(super) fn message(&self, opening: &[u8; ENCODED_SIZE]) -> Option<Vec<u8>> {
        let mut message = self.prefix[..OPENING_START].to_vec();
        message.extend(opening);
        if message[..self.prefix.len()] != self.prefix
            || opening[CONTENT_SIZE..].iter().any(|&v| v != 0)
        {
            return None;
        }
        if NONZERO
            .iter()
            .any(|&(start, end)| opening[start..end].iter().all(|&v| v == 0))
        {
            return None;
        }
        Some(message)
    }
}

pub(super) fn nonzero_inverses(opening: &[u8; ENCODED_SIZE]) -> [Value; 4] {
    NONZERO.map(|(start, end)| {
        let count: u32 = opening[start..end].iter().map(|v| v.count_ones()).sum();
        Value::from_u32(count).inverse()
    })
}
