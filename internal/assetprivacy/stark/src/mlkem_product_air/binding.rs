// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{A0, B0, C0, GAMMA, LEFT, MlKemProductAir, OUTPUT, RIGHT};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MlKemProductAir {
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for start in [LEFT, RIGHT, OUTPUT] {
            for index in 0..256 {
                builder.when_transition().assert_eq(
                    main.current_slice()[start + index],
                    main.next_slice()[start + index],
                );
            }
        }
        let gamma = builder.preprocessed().current_slice()[128];
        builder.assert_eq(main.current_slice()[GAMMA], gamma);
        for pair in 0..128 {
            self.eval_pair(builder, pair);
        }
    }

    fn eval_pair<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, pair: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[pair];
        for lane in 0..2 {
            for (poly, value) in [(LEFT, A0), (RIGHT, B0), (OUTPUT, C0)] {
                builder
                    .when(flag)
                    .assert_eq(row[poly + 2 * pair + lane], row[value + 25 * lane]);
            }
        }
    }
}
