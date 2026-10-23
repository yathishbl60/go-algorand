// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{NUM_KECCAK_COLS, NUM_ROUNDS, generate_trace_rows};
use p3_matrix::{Matrix, dense::RowMajorMatrix};

use super::Sha3Air;
use crate::Value;

impl Sha3Air {
    /// Generates private trace data; rejects a message with the wrong fixed length.
    pub fn trace(&self, message: &[u8]) -> Option<RowMajorMatrix<Value>> {
        if message.len() != self.message_len {
            return None;
        }
        let keccak = generate_trace_rows(self.permutation_inputs(message), 0);
        let mut values = Value::zero_vec(keccak.height() * self.width());
        for (r, row) in values.chunks_exact_mut(self.width()).enumerate() {
            row[..NUM_KECCAK_COLS]
                .copy_from_slice(&keccak.values[r * NUM_KECCAK_COLS..][..NUM_KECCAK_COLS]);
            self.write_message(row, message);
            if r / NUM_ROUNDS < self.blocks() {
                row[self.block_offset() + r / NUM_ROUNDS] = Value::ONE;
            }
        }
        Some(RowMajorMatrix::new(values, self.width()))
    }

    fn write_message(&self, row: &mut [Value], message: &[u8]) {
        for (i, byte) in message.iter().enumerate() {
            for bit in 0..8 {
                row[NUM_KECCAK_COLS + i * 8 + bit] = Value::from_bool(byte & (1 << bit) != 0);
            }
        }
    }
}

/// Converts a SHA3-512 digest into the verifier's 32 little-endian uint16 limbs.
pub fn digest_values(digest: [u8; 64]) -> [Value; 32] {
    core::array::from_fn(|i| {
        Value::from_u16(u16::from_le_bytes([digest[2 * i], digest[2 * i + 1]]))
    })
}
