// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Value, aes_air::Byte};
use p3_air::AirBuilder;
use p3_field::{PrimeCharacteristicRing, PrimeField64};

pub(super) fn eval_xor<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    left: Byte<AB::Var>,
    right: Byte<AB::Var>,
    output: Byte<AB::Var>,
) {
    for bit in 0..8 {
        builder.assert_eq(
            output[bit],
            AB::Expr::from(left[bit]).xor(&right[bit].into()),
        );
    }
}

pub(super) fn bits(value: u8) -> Byte<Value> {
    core::array::from_fn(|bit| Value::from_bool(value & (1 << bit) != 0))
}

pub(super) fn integer(value: Byte<Value>) -> u8 {
    value.into_iter().enumerate().fold(0, |byte, (bit, value)| {
        byte | ((value.as_canonical_u64() as u8) << bit)
    })
}

pub(super) fn xor(left: Byte<Value>, right: Byte<Value>) -> Byte<Value> {
    bits(integer(left) ^ integer(right))
}
