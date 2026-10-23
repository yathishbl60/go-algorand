// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Block,
    block::{eval_block, eval_shift},
};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// 49,280 Boolean columns. The exact product is sums[127].
pub struct Gf128<T> {
    pub left: Block<T>,
    pub right: Block<T>,
    pub powers: [Block<T>; 127],
    pub selected: [Block<T>; 128],
    pub sums: [Block<T>; 128],
}

pub fn eval_multiply<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gf128<AB::Var>) {
    eval_block(builder, value.left);
    eval_block(builder, value.right);
    for step in 0..128 {
        let power = if step == 0 {
            value.right
        } else {
            value.powers[step - 1]
        };
        if step < 127 {
            eval_shift(builder, power, value.powers[step]);
        }
        eval_block(builder, value.selected[step]);
        eval_block(builder, value.sums[step]);
        eval_step(builder, value, step, power);
    }
}

fn eval_step<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    value: &Gf128<AB::Var>,
    step: usize,
    power: Block<AB::Var>,
) {
    for (bit, power_bit) in power.into_iter().enumerate() {
        builder.assert_eq(value.selected[step][bit], power_bit * value.left[step]);
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
