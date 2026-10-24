// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{KEY_BYTES, MlKemDerivationAir};
use p3_keccak_air::NUM_KECCAK_COLS;

impl MlKemDerivationAir {
    pub(super) fn digest_start(&self) -> usize {
        self.key_hash.width() + self.expand.width()
    }
    pub(super) fn output_start(&self) -> usize {
        self.digest_start() + 48
    }
    pub(super) fn scratch(&self) -> usize {
        self.output_start() + 512
    }
    pub(crate) fn width(&self) -> usize {
        self.scratch() + 24
    }
    pub(super) fn expand_message(&self) -> usize {
        self.key_hash.width() + NUM_KECCAK_COLS
    }

    /// The 1184-byte private canonical encapsulation key in LSB-first byte bits.
    pub fn key_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == self.width()).then(|| &row[NUM_KECCAK_COLS..NUM_KECCAK_COLS + KEY_BYTES * 8])
    }
    pub fn rho_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        Some(&self.key_bits(row)?[1152 * 8..])
    }
    pub fn message_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == self.width())
            .then(|| &row[self.expand_message()..self.expand_message() + 256])
    }
    pub fn shared_secret_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == self.width()).then(|| &row[self.output_start()..self.output_start() + 256])
    }
    pub fn coins_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == self.width()).then(|| &row[self.output_start() + 256..self.scratch()])
    }
}
