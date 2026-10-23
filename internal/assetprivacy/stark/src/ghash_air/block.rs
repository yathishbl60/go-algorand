// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// Index 0 is the most significant bit of the first transmitted byte.
pub type Block<T> = [T; 128];

/// Reorders existing AES byte-bit columns without introducing new witness data.
pub fn from_aes_bits<T: Copy>(bytes: [[T; 8]; 16]) -> Block<T> {
    core::array::from_fn(|bit| bytes[bit / 8][7 - bit % 8])
}

/// Reorders existing GHASH bit columns into the AES component's byte-bit layout.
pub fn to_aes_bits<T: Copy>(block: Block<T>) -> [[T; 8]; 16] {
    core::array::from_fn(|byte| core::array::from_fn(|bit| block[byte * 8 + 7 - bit]))
}

pub(super) fn eval_block<AB: AirBuilder<F = Value>>(builder: &mut AB, value: Block<AB::Var>) {
    builder.assert_bools(value);
}

pub(super) fn bits(value: u128) -> Block<Value> {
    core::array::from_fn(|bit| Value::from_bool(value & (1 << (127 - bit)) != 0))
}

pub(super) fn eval_shift<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    input: Block<AB::Var>,
    output: Block<AB::Var>,
) {
    eval_block(builder, output);
    for bit in 0..128 {
        let shifted = if bit == 0 {
            AB::Expr::ZERO
        } else {
            input[bit - 1].into()
        };
        let expected = if matches!(bit, 0 | 1 | 2 | 7) {
            shifted.xor(&input[127].into())
        } else {
            shifted
        };
        builder.assert_eq(output[bit], expected);
    }
}
