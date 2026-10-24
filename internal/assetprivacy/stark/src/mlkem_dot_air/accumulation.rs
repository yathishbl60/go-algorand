// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACC_OUTPUT, END, MlKemDotAir, OUTPUT, STATE};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemDotAir {
    pub(super) fn eval_accumulation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..256 {
            self.eval_state(builder, index);
        }
        for dot in 0..4 {
            for index in 0..256 {
                self.eval_output(builder, dot, index);
            }
        }
    }
    fn eval_state<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let reset: AB::Expr = builder.preprocessed().current_slice()[END..END + 4]
            .iter()
            .map(|&v| v.into())
            .sum();
        let next = (AB::Expr::ONE - reset) * updated::<AB>(builder, index);
        builder.when_first_row().assert_zero(row[STATE + index]);
        builder
            .when_transition()
            .assert_eq(main.next_slice()[STATE + index], next);
    }
    fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, dot: usize, index: usize) {
        let value = updated::<AB>(builder, index);
        let gate = builder.preprocessed().current_slice()[END + dot];
        let main = builder.main();
        builder
            .when(gate)
            .assert_eq(main.current_slice()[OUTPUT + dot * 256 + index], value);
    }
}

fn updated<AB: AirBuilder<F = Value>>(builder: &AB, index: usize) -> AB::Expr {
    let main = builder.main();
    let row = main.current_slice();
    let current = row[STATE + index];
    let selected = builder.preprocessed().current_slice()[index / 2];
    let output = row[ACC_OUTPUT + 25 * (index % 2)];
    current + selected * (output - current)
}
