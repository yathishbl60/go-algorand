// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// Exactly eight Boolean bits, least significant first.
pub type Byte<T> = [T; 8];

pub fn eval_byte<AB: AirBuilder<F = Value>>(builder: &mut AB, value: Byte<AB::Var>) {
    builder.assert_bools(value);
}

/// AES multiplication by x modulo x^8+x^4+x^3+x+1, with ranged inputs/output.
pub fn eval_xtime<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    input: Byte<AB::Var>,
    output: Byte<AB::Var>,
) {
    eval_byte(builder, input);
    eval_byte(builder, output);
    builder.assert_eq(output[0], input[7]);
    for bit in 1..8 {
        let shifted: AB::Expr = input[bit - 1].into();
        let expected = if 0x1b & (1 << bit) != 0 {
            shifted.xor(&input[7].into())
        } else {
            shifted
        };
        builder.assert_eq(output[bit], expected);
    }
}

pub(super) fn bits(value: u8) -> Byte<Value> {
    core::array::from_fn(|bit| Value::from_bool(value & (1 << bit) != 0))
}
