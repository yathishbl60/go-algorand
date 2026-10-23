// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OUTPUT, SEED, Shake128MatrixAir};

/// Private byte-order concatenation, with LSB-first bits within every byte.
pub struct Wires<'a, T> {
    pub seed_bits: &'a [T],
    pub output_bits: &'a [T],
}

impl Shake128MatrixAir {
    pub fn wires<'a, T>(&self, row: &'a [T]) -> Option<Wires<'a, T>> {
        (row.len() == self.trace_width()).then(|| Wires {
            seed_bits: &row[SEED..OUTPUT],
            output_bits: &row[OUTPUT..self.trace_width()],
        })
    }
    pub fn seed_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        (byte < 32 && bit < 8).then(|| SEED + byte * 8 + bit)
    }
    pub fn output_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        (byte < self.output_bytes() && bit < 8).then(|| OUTPUT + byte * 8 + bit)
    }
}
