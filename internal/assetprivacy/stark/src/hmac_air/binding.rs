// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK_BYTES, HmacAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl HmacAir {
    pub(super) fn eval_keys<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let row = window.current_slice();
        for (start, pad) in [(0, 0x36u8), (self.inner.width(), 0x5c)] {
            for byte in 0..BLOCK_BYTES {
                for bit in 0..8 {
                    let expected = self.padded_key_bit::<AB>(builder, row, byte, bit, pad);
                    builder.assert_eq(row[start + NUM_KECCAK_COLS + byte * 8 + bit], expected);
                }
            }
        }
    }

    fn padded_key_bit<AB: AirBuilder<F = Value>>(
        &self,
        builder: &mut AB,
        row: &[AB::Var],
        byte: usize,
        bit: usize,
        pad: u8,
    ) -> AB::Expr {
        let padding = AB::Expr::from_bool(pad & (1 << bit) != 0);
        if byte >= 64 {
            return padding;
        }
        let key = row[self.key_start() + byte * 8 + bit];
        builder.assert_bool(key);
        AB::Expr::from(key).xor(&padding)
    }

    pub(super) fn eval_inner_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let row = window.current_slice();
        let start = self.inner.width() + NUM_KECCAK_COLS + BLOCK_BYTES * 8;
        for limb in 0..32 {
            let value = limb_value::<AB>(&row[start + limb * 16..][..16]);
            builder.assert_eq(row[self.digest_start() + limb], value);
        }
    }

    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let row = window.current_slice();
        let outputs = &row[self.output_start()..self.width()];
        let next = &window.next_slice()[self.output_start()..self.width()];
        for (&current, &next) in outputs.iter().zip(next) {
            builder.assert_bool(current);
            builder.when_transition().assert_eq(current, next);
        }
        for limb in 0..32 {
            let start = self.output_start() + limb * 16;
            let value = limb_value::<AB>(&row[start..start + 16]);
            builder.assert_eq(row[self.digest_start() + 32 + limb], value);
        }
    }
}

fn limb_value<AB: AirBuilder<F = Value>>(bits: &[AB::Var]) -> AB::Expr {
    bits.iter()
        .rev()
        .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit)
}
