// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    BORROW, CARRY, HIGH, INPUT, LEFT, LOW, MlKemNttAir, OUTPUT, PRODUCT, QUOTIENT, RIGHT, STATE,
    view,
};
use crate::{Value, mlkem_field::eval_canonical};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemNttAir {
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for index in 0..256 {
            builder
                .when_first_row()
                .assert_eq(row[INPUT + index], row[STATE + index]);
            builder
                .when_last_row()
                .assert_eq(row[OUTPUT + index], row[STATE + index]);
        }
        self.eval_constants(builder);
        for start in [LEFT, RIGHT, PRODUCT, QUOTIENT, LOW, HIGH] {
            eval_canonical(builder, &view::canonical(row, start));
        }
        builder.assert_bool(row[CARRY]);
        builder.assert_bool(row[BORROW]);
    }

    fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for start in [INPUT, OUTPUT] {
            for index in 0..256 {
                builder
                    .when_transition()
                    .assert_eq(row[start + index], main.next_slice()[start + index]);
            }
        }
    }

    pub(super) fn eval_operands<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let left: AB::Expr = (0..256)
            .map(|i| row[STATE + i] * prep.current_slice()[i])
            .sum();
        let right: AB::Expr = (0..256)
            .map(|i| row[STATE + i] * prep.current_slice()[256 + i])
            .sum();
        builder.assert_eq(row[LEFT], left);
        builder.assert_eq(row[RIGHT], right);
    }
}
