// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCEPT, CANDIDATE, CURSOR, GAP, MlKemStreamMatrixAir, OUTPUT, REJECT, binding::bits};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemStreamMatrixAir {
    pub(super) fn eval_cursor<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &flag in &row[CURSOR..CANDIDATE] {
            builder.assert_bool(flag);
        }
        let count: AB::Expr = row[CURSOR..CANDIDATE].iter().copied().map(Into::into).sum();
        builder.assert_one(count);
        builder.when_first_row().assert_one(row[CURSOR]);
        builder.when_last_row().assert_one(row[CURSOR + 256]);
        self.eval_accept(builder);
        self.eval_step(builder);
    }
    fn eval_accept<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active: AB::Expr = builder.preprocessed().current_slice()[..112]
            .iter()
            .copied()
            .map(Into::into)
            .sum();
        builder.assert_bool(row[ACCEPT]);
        builder.assert_eq(
            row[ACCEPT],
            active * (AB::Expr::ONE - row[REJECT]) * (AB::Expr::ONE - row[CURSOR + 256]),
        );
    }
    fn eval_step<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for index in 0..=256 {
            let prior: AB::Expr = if index == 0 {
                AB::Expr::ZERO
            } else {
                row[CURSOR + index - 1].into()
            };
            let current = row[CURSOR + index];
            builder.when_transition().assert_eq(
                main.next_slice()[CURSOR + index],
                current + row[ACCEPT] * (prior - current),
            );
        }
    }
    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let selected: AB::Expr = (0..256)
            .map(|index| row[OUTPUT + index] * row[CURSOR + index])
            .sum();
        builder
            .when(row[ACCEPT])
            .assert_eq(selected, bits::<AB>(&row[CANDIDATE..GAP]));
    }
}
