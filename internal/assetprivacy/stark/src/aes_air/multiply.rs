// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Byte, eval_byte, eval_xtime};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// 200 Boolean columns. The product is sums[7]; no duplicate output is trusted.
pub struct GfMul<T> {
    pub left: Byte<T>,
    pub right: Byte<T>,
    pub powers: [Byte<T>; 7],
    pub selected: [Byte<T>; 8],
    pub sums: [Byte<T>; 8],
}

pub fn eval_multiply<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &GfMul<AB::Var>) {
    eval_byte(builder, value.left);
    eval_byte(builder, value.right);
    for step in 0..8 {
        let power = if step == 0 {
            value.left
        } else {
            value.powers[step - 1]
        };
        if step < 7 {
            eval_xtime(builder, power, value.powers[step]);
        }
        eval_byte(builder, value.selected[step]);
        eval_byte(builder, value.sums[step]);
        eval_step(builder, value, step, power);
    }
}

fn eval_step<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    value: &GfMul<AB::Var>,
    step: usize,
    power: Byte<AB::Var>,
) {
    for (bit, power_bit) in power.into_iter().enumerate() {
        builder.assert_eq(value.selected[step][bit], power_bit * value.right[step]);
        let prior = if step == 0 {
            AB::Expr::ZERO
        } else {
            value.sums[step - 1][bit].into()
        };
        builder.assert_eq(
            value.sums[step][bit],
            prior.xor(&value.selected[step][bit].into()),
        );
    }
}
