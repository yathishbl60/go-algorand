// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    A0, ACC_INPUT, B0, CORE, GAMMA, MlKemDotAir, OP, PRE_GAMMA, STATE, WORK, inputs::source,
};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MlKemDotAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (&current, &next) in main.current_slice()[..CORE]
            .iter()
            .zip(&main.next_slice()[..CORE])
        {
            builder.when_transition().assert_eq(current, next);
        }
        let gamma = builder.preprocessed().current_slice()[PRE_GAMMA];
        builder.assert_eq(main.current_slice()[GAMMA], gamma);
    }
    pub(super) fn eval_polynomials<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..256 {
            let left = selected_polynomial::<AB>(builder, index, false);
            let right = selected_polynomial::<AB>(builder, index, true);
            let main = builder.main();
            builder.assert_eq(main.current_slice()[WORK + index], left);
            builder.assert_eq(main.current_slice()[WORK + 256 + index], right);
        }
    }
    pub(super) fn eval_operands<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for lane in 0..2 {
            for (target, start) in [(A0, WORK), (B0, WORK + 256), (ACC_INPUT, STATE)] {
                let selected = selected_pair::<AB>(builder, start, lane);
                let main = builder.main();
                builder.assert_eq(main.current_slice()[target + 25 * lane], selected);
            }
        }
    }
}

fn selected_polynomial<AB: AirBuilder<F = Value>>(
    builder: &AB,
    index: usize,
    right: bool,
) -> AB::Expr {
    let main = builder.main();
    let prep = builder.preprocessed();
    (0..12)
        .map(|op| prep.current_slice()[OP + op] * main.current_slice()[source(op, right) + index])
        .sum()
}

fn selected_pair<AB: AirBuilder<F = Value>>(builder: &AB, start: usize, lane: usize) -> AB::Expr {
    let main = builder.main();
    let prep = builder.preprocessed();
    (0..128)
        .map(|pair| prep.current_slice()[pair] * main.current_slice()[start + 2 * pair + lane])
        .sum()
}
