// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::HkdfAir;

impl HkdfAir {
    pub fn shared_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        self.extract.data_bit_column(byte, bit)
    }

    pub fn salt_bits<T: Copy>(&self, row: &[T]) -> Option<[[T; 8]; 64]> {
        if row.len() != self.width() {
            return None;
        }
        self.extract.key_bits(&row[..self.extract.width()])
    }

    pub fn output_bits<T: Copy>(&self, row: &[T]) -> Option<[[T; 8]; 32]> {
        if row.len() != self.width() {
            return None;
        }
        let output = self.expand.output_bits(&row[self.extract.width()..])?;
        Some(core::array::from_fn(|byte| output[byte]))
    }
}
