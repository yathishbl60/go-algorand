// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

use super::Sha3Air;
use crate::Value;

impl Sha3Air {
    /// Private witness construction; rejects a message with the wrong fixed length.
    pub fn trace(&self, message: &[u8]) -> Option<RowMajorMatrix<Value>> {
        if message.len() != self.message_len {
            return None;
        }
        let mut values = Value::zero_vec(self.trace_rows() * self.width());
        self.write_into(message, &mut values, self.width(), 0)?;
        Some(RowMajorMatrix::new(values, self.width()))
    }
}

/// Converts a SHA3-512 digest into the verifier's 32 little-endian uint16 limbs.
pub fn digest_values(digest: [u8; 64]) -> [Value; 32] {
    core::array::from_fn(|i| {
        Value::from_u16(u16::from_le_bytes([digest[2 * i], digest[2 * i + 1]]))
    })
}

/// SHA3-256 uses 16 little-endian uint16 limbs, with no zero extension.
pub fn digest256_values(digest: [u8; 32]) -> [Value; 16] {
    core::array::from_fn(|i| {
        Value::from_u16(u16::from_le_bytes([digest[2 * i], digest[2 * i + 1]]))
    })
}
