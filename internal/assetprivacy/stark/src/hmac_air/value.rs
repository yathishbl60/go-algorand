// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK_BYTES, HmacAir};
use p3_keccak_air::NUM_KECCAK_COLS;

impl HmacAir {
    pub fn data_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        (byte < self.data_len && bit < 8).then(|| NUM_KECCAK_COLS + (BLOCK_BYTES + byte) * 8 + bit)
    }

    pub fn key_bits<T: Copy>(&self, row: &[T]) -> Option<[[T; 8]; 64]> {
        (row.len() == self.width()).then(|| {
            core::array::from_fn(|byte| {
                core::array::from_fn(|bit| row[self.key_start() + byte * 8 + bit])
            })
        })
    }

    pub fn output_bits<T: Copy>(&self, row: &[T]) -> Option<[[T; 8]; 64]> {
        (row.len() == self.width()).then(|| {
            core::array::from_fn(|byte| {
                core::array::from_fn(|bit| row[self.output_start() + byte * 8 + bit])
            })
        })
    }
}
