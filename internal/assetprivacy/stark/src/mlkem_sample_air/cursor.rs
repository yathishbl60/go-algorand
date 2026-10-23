// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemSampleAir {
    pub(super) fn eval_cursor<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let cursor = &row[self.cursor_start()..self.candidate_start()];
        for &flag in cursor {
            builder.assert_bool(flag);
        }
        let count: AB::Expr = cursor.iter().copied().map(Into::into).sum();
        builder.assert_one(count);
        builder.when_first_row().assert_one(cursor[0]);
        builder.when_last_row().assert_one(cursor[256]);
        builder.assert_bool(row[self.accept_column()]);
        builder.assert_eq(
            row[self.accept_column()],
            self.active::<AB>(builder)
                * (AB::Expr::ONE - row[self.reject_column()])
                * (AB::Expr::ONE - cursor[256]),
        );
        self.eval_cursor_step(builder);
    }

    fn eval_cursor_step<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let accepted = row[self.accept_column()];
        for index in 0..=256 {
            let prior: AB::Expr = if index == 0 {
                AB::Expr::ZERO
            } else {
                row[self.cursor_start() + index - 1].into()
            };
            let current = row[self.cursor_start() + index];
            let next = current + accepted * (prior - current);
            builder
                .when_transition()
                .assert_eq(main.next_slice()[self.cursor_start() + index], next);
        }
    }

    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let selected: AB::Expr = (0..256)
            .map(|index| row[self.output_start() + index] * row[self.cursor_start() + index])
            .sum();
        builder
            .when(row[self.accept_column()])
            .assert_eq(selected, self.candidate_value::<AB>(row));
    }
}
