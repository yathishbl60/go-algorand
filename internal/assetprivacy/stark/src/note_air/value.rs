// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_keccak_air::NUM_KECCAK_COLS;

use super::{NoteCommitmentAir, OPENING_START};

impl NoteCommitmentAir {
    /// Shares the exact committed account bytes, with each byte's bits LSB first.
    pub fn account_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 256]> {
        if row.len() != self.width() {
            return None;
        }
        Some(core::array::from_fn(|bit| {
            row[self.opening_bit_column(137 + bit / 8, bit % 8)]
        }))
    }

    pub(crate) fn opening_bit_column(&self, byte: usize, bit: usize) -> usize {
        NUM_KECCAK_COLS + (OPENING_START + byte) * 8 + bit
    }

    /// Shares the committed uint64 value bits with the conservation constraints.
    /// Output is least-significant bit first, matching `amounts::Amounts`.
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        if row.len() != self.width() {
            return None;
        }
        Some(core::array::from_fn(|bit| {
            let byte = OPENING_START + 176 - bit / 8;
            row[NUM_KECCAK_COLS + byte * 8 + bit % 8]
        }))
    }
}
