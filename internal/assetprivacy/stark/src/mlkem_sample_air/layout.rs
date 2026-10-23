// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_air::BaseAir;

impl MlKemSampleAir {
    pub fn prefix_bytes(&self) -> usize {
        self.rate_blocks * 168
    }
    pub fn trace_rows(&self) -> usize {
        (self.candidate_count() + 1).next_power_of_two().max(512)
    }
    pub(super) fn candidate_count(&self) -> usize {
        self.rate_blocks * 112
    }
    pub(super) fn output_start(&self) -> usize {
        self.prefix_bytes() * 8
    }
    pub(super) fn cursor_start(&self) -> usize {
        self.output_start() + 256
    }
    pub(super) fn candidate_start(&self) -> usize {
        self.cursor_start() + 257
    }
    pub(super) fn low_start(&self) -> usize {
        self.candidate_start() + 12
    }
    pub(super) fn reject_column(&self) -> usize {
        self.low_start() + 12
    }
    pub(super) fn accept_column(&self) -> usize {
        self.reject_column() + 1
    }

    /// Private LSB-first byte bits; an enclosing AIR must bind every prefix bit.
    pub fn prefix_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        if byte >= self.prefix_bytes() || bit >= 8 {
            return None;
        }
        Some(byte * 8 + bit)
    }

    /// Private canonical residues, each linked to its first accepted candidate.
    pub fn coefficients<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != <Self as BaseAir<Value>>::width(self) {
            return None;
        }
        row[self.output_start()..self.cursor_start()]
            .try_into()
            .ok()
    }
}
