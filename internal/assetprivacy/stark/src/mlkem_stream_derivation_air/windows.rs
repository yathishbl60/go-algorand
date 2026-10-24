// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK, MlKemStreamDerivationAir, RHO, WINDOW};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemStreamDerivationAir {
    pub(super) fn eval_windows<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for &bit in &main.current_slice()[WINDOW..WINDOW + 1104] {
            builder.assert_bool(bit);
        }
        for block in 0..9 {
            self.eval_window(builder, block);
        }
    }
    fn eval_window<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[BLOCK + block];
        let first = block * 1088 / 12;
        for coefficient in 0..92 {
            if first + coefficient < 768 {
                let start = WINDOW + 12 * coefficient;
                let value = super::message::bits::<AB>(&row[start..start + 12]);
                builder
                    .when(active)
                    .assert_eq(value, row[first + coefficient]);
            }
        }
        self.eval_window_tail(builder, block, first);
    }
    fn eval_window_tail<AB: AirBuilder<F = Value>>(
        &self,
        builder: &mut AB,
        block: usize,
        first: usize,
    ) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[BLOCK + block];
        for bit in 9216usize.saturating_sub(first * 12)..1104 {
            let global = first * 12 + bit;
            let expected = if global < 9472 {
                row[RHO + global - 9216].into()
            } else {
                AB::Expr::ZERO
            };
            builder.when(active).assert_eq(row[WINDOW + bit], expected);
        }
    }
}
