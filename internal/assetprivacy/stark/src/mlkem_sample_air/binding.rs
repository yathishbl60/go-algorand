// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemSampleAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[..self.output_start()] {
            builder.assert_bool(bit);
        }
        for (current, next) in row[..self.cursor_start()]
            .iter()
            .zip(&main.next_slice()[..self.cursor_start()])
        {
            builder.when_transition().assert_eq(*current, *next);
        }
    }

    pub(super) fn eval_candidate<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for bit in 0..12 {
            let schedule = builder.preprocessed();
            let selected: AB::Expr = (0..self.candidate_count())
                .map(|index| schedule.current_slice()[index] * row[12 * index + bit])
                .sum();
            builder.assert_bool(row[self.candidate_start() + bit]);
            builder.assert_eq(row[self.candidate_start() + bit], selected);
        }
    }

    pub(super) fn eval_comparison<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[self.low_start()..self.reject_column()] {
            builder.assert_bool(bit);
        }
        builder.assert_bool(row[self.reject_column()]);
        builder.assert_eq(
            self.candidate_value::<AB>(row) + Value::from_u16(767),
            row[self.reject_column()] * Value::from_u16(4096)
                + bits::<AB>(&row[self.low_start()..self.reject_column()]),
        );
    }

    pub(super) fn candidate_value<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var]) -> AB::Expr {
        bits::<AB>(&row[self.candidate_start()..self.low_start()])
    }
}

fn bits<AB: AirBuilder<F = Value>>(bits: &[AB::Var]) -> AB::Expr {
    bits.iter()
        .enumerate()
        .map(|(index, &bit)| bit * Value::from_u16(1 << index))
        .sum()
}
