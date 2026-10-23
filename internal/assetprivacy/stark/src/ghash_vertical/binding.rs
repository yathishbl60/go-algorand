// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCUMULATOR, BITS, GhashProductAir, INPUT, LEFT, OUTPUT, POWER, RIGHT, SELECTED};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl GhashProductAir {
    pub(super) fn eval_ranges<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for &bit in main.current_slice() {
            builder.assert_bool(bit);
        }
        for start in [LEFT, RIGHT, OUTPUT] {
            for bit in 0..BITS {
                builder.when_transition().assert_eq(
                    main.current_slice()[start + bit],
                    main.next_slice()[start + bit],
                );
            }
        }
    }

    pub(super) fn eval_boundaries<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let final_step = builder.preprocessed().current_slice()[BITS - 1];
        for bit in 0..BITS {
            builder
                .when_first_row()
                .assert_eq(row[POWER + bit], row[RIGHT + bit]);
            builder.when_first_row().assert_zero(row[ACCUMULATOR + bit]);
            let product = AB::Expr::from(row[ACCUMULATOR + bit]).xor(&row[SELECTED + bit].into());
            builder
                .when(final_step)
                .assert_eq(row[OUTPUT + bit], product);
        }
    }

    pub(super) fn eval_selection<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let schedule = builder.preprocessed();
        let input: AB::Expr = (0..BITS)
            .map(|bit| row[LEFT + bit] * schedule.current_slice()[bit])
            .sum();
        builder.assert_eq(row[INPUT], input);
        for bit in 0..BITS {
            builder.assert_eq(row[SELECTED + bit], row[POWER + bit] * row[INPUT]);
        }
    }
}
