// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, BITS, BLOCKS, DATA, GhashChainAir, H, INPUT, RESULT, SELECTED, V, X, Y, Z};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl GhashChainAir {
    pub(super) fn eval_ranges<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for &bit in main.current_slice() {
            builder.assert_bool(bit);
        }
        for (start, width) in [(H, BITS), (AAD, 512), (RESULT, BITS)] {
            for bit in 0..width {
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
        let prep = builder.preprocessed();
        let start = prep.current_slice()[0];
        let last = prep.current_slice()[BITS - 1] * prep.current_slice()[BITS + BLOCKS - 1];
        for bit in 0..BITS {
            builder.when_first_row().assert_zero(row[Y + bit]);
            builder.when(start).assert_eq(row[V + bit], row[H + bit]);
            builder.when(start).assert_zero(row[Z + bit]);
            builder.when(start).assert_eq(
                row[X + bit],
                AB::Expr::from(row[Y + bit]).xor(&row[DATA + bit].into()),
            );
            let product = AB::Expr::from(row[Z + bit]).xor(&row[SELECTED + bit].into());
            builder
                .when(last.clone())
                .assert_eq(row[RESULT + bit], product);
        }
    }

    pub(super) fn eval_selection<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let input: AB::Expr = (0..BITS)
            .map(|i| row[X + i] * prep.current_slice()[i])
            .sum();
        builder.assert_eq(row[INPUT], input);
        for bit in 0..BITS {
            builder.assert_eq(row[SELECTED + bit], row[V + bit] * row[INPUT]);
        }
    }
}
