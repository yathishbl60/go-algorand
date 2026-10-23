// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemNoiseAir;
use crate::Value;
use p3_air::BaseAir;

impl MlKemNoiseAir {
    pub(super) fn digest_start(&self) -> usize {
        self.hash.width()
    }
    pub(super) fn bit_start(&self) -> usize {
        self.digest_start() + 64
    }
    pub(super) fn flag_start(&self) -> usize {
        self.bit_start() + 1024
    }
    pub(super) fn coefficient_start(&self) -> usize {
        self.flag_start() + 1280
    }

    /// Shares the seed's byte-bit columns with later encryption-randomness binding.
    pub fn seed_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        if byte >= 32 || bit >= 8 {
            return None;
        }
        self.hash.message_bit_column(byte, bit)
    }

    /// Canonical residues in {3327,3328,0,1,2}, ordered by polynomial index.
    pub fn coefficients<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != <Self as BaseAir<Value>>::width(self) {
            return None;
        }
        row[self.coefficient_start()..].try_into().ok()
    }

    pub fn trace_rows(&self) -> usize {
        self.hash.trace_rows()
    }
}
