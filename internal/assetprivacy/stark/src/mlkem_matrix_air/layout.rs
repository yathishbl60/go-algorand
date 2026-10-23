// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemMatrixAir;
use crate::Value;
use p3_air::BaseAir;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MlKemMatrixAir {
    /// Private rho bits in LSB-first byte order. No public matrix seed is exposed.
    pub fn seed_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != <Self as BaseAir<Value>>::width(self) {
            return None;
        }
        row[NUM_KECCAK_COLS..self.sampler_start()].try_into().ok()
    }
    /// The exact first256 accepted SHAKE128 coefficients, in canonical 0..3328.
    pub fn coefficients<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != <Self as BaseAir<Value>>::width(self) {
            return None;
        }
        self.sample.coefficients(&row[self.sampler_start()..])
    }
}
