// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, MembershipVerticalAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MembershipVerticalAir {
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let active = builder.preprocessed().current_slice()[FLAGS];
        for (byte, &value) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                let wire = main.current_slice()[NUM_KECCAK_COLS + byte * 8 + bit];
                builder
                    .when(active)
                    .assert_eq(wire, Value::from_bool(value & (1 << bit) != 0));
            }
        }
        self.eval_level(builder);
    }

    fn eval_level<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let preprocessing = builder.preprocessed().clone();
        let row = preprocessing.current_slice();
        for bit in 0..8 {
            let expected: AB::Expr = (0..32)
                .map(|level| row[level] * Value::from_bool(level & (1 << bit) != 0))
                .sum();
            let column = NUM_KECCAK_COLS + 148 * 8 + bit;
            builder
                .when(row[FLAGS])
                .assert_eq(main.current_slice()[column], expected);
        }
    }
}
