// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemEncodingAir, PACKED};

/// Private coefficients and contiguous LSB-first packed bits.
pub struct Wires<'a, T> {
    pub coefficients: &'a [T],
    pub packed_bits: &'a [T],
}

impl MlKemEncodingAir {
    pub fn wires<'a, T>(&self, row: &'a [T]) -> Option<Wires<'a, T>> {
        (row.len() == self.trace_width()).then(|| Wires {
            coefficients: &row[COEFFICIENTS..PACKED],
            packed_bits: &row[PACKED..self.scratch()],
        })
    }

    pub fn bit_column(&self, coefficient: usize, bit: usize) -> Option<usize> {
        if coefficient >= 256 || bit >= self.bits {
            return None;
        }
        Some(PACKED + coefficient * self.bits + bit)
    }

    /// The returned eight bits encode one byte, from least to most significant.
    pub fn byte_bits<'a, T>(&self, row: &'a [T], byte: usize) -> Option<&'a [T]> {
        if row.len() != self.trace_width() || byte >= self.encoded_bytes() {
            return None;
        }
        Some(&row[PACKED + 8 * byte..PACKED + 8 * byte + 8])
    }
}
