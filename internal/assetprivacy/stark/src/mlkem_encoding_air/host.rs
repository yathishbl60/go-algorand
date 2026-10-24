// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncodingAir, write};
use crate::mlkem_field::Q;

impl MlKemEncodingAir {
    /// Packs canonical coefficients using the trace's exact rounding arithmetic.
    pub fn encode(&self, input: [u16; 256]) -> Option<Vec<u8>> {
        if !self.valid_input(&input) {
            return None;
        }
        let mut bytes = vec![0; self.encoded_bytes()];
        for (index, coefficient) in input.into_iter().enumerate() {
            let encoded = write::encoded(coefficient, self.bits);
            for bit in 0..self.bits {
                let position = index * self.bits + bit;
                bytes[position / 8] |= (((encoded >> bit) & 1) as u8) << (position % 8);
            }
        }
        Some(bytes)
    }

    /// Exact inverse for D=12 canonical encoding or D=1 message decompression.
    pub fn decode(&self, bytes: &[u8]) -> Option<[u16; 256]> {
        if !matches!(self.bits, 1 | 12) || bytes.len() != self.encoded_bytes() {
            return None;
        }
        let output = core::array::from_fn(|index| {
            let encoded: u16 = (0..self.bits)
                .map(|bit| input_bit(bytes, index * self.bits + bit) << bit)
                .sum();
            if self.bits == 1 {
                encoded * 1665
            } else {
                encoded
            }
        });
        self.valid_input(&output).then_some(output)
    }

    pub(super) fn valid_input(&self, input: &[u16; 256]) -> bool {
        input
            .iter()
            .all(|&x| x < Q && (self.bits != 1 || matches!(x, 0 | 1665)))
    }
}

fn input_bit(bytes: &[u8], index: usize) -> u16 {
    u16::from((bytes[index / 8] >> (index % 8)) & 1)
}
