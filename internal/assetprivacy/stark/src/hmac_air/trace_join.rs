// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HmacAir, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

impl HmacAir {
    pub(super) fn write_auxiliary(
        &self,
        values: &mut [Value],
        stride: usize,
        column: usize,
        key: &[u8; 64],
        input: &Prepared,
    ) {
        for parent in values.chunks_exact_mut(stride) {
            let row = &mut parent[column..column + self.width()];
            let start = self.digest_start();
            row[start..start + 32].copy_from_slice(&digest_values(input.inner_digest));
            row[start + 32..start + 64].copy_from_slice(&digest_values(input.output));
            self.write_bits(row, key, &input.output);
        }
    }

    fn write_bits(&self, row: &mut [Value], key: &[u8; 64], output: &[u8; 64]) {
        for byte in 0..64 {
            for bit in 0..8 {
                let index = byte * 8 + bit;
                row[self.key_start() + index] = Value::from_bool(key[byte] & (1 << bit) != 0);
                row[self.output_start() + index] = Value::from_bool(output[byte] & (1 << bit) != 0);
            }
        }
    }
}
