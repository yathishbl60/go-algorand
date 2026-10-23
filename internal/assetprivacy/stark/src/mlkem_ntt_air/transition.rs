// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HIGH, LOW, MlKemNttAir, STATE};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MlKemNttAir {
    pub(super) fn eval_transition<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for index in 0..256 {
            let prep = builder.preprocessed();
            let current = row[STATE + 25 * index];
            let left = prep.current_slice()[index] * (row[LOW] - current);
            let right = prep.current_slice()[256 + index] * (row[HIGH] - current);
            builder.when_transition().assert_eq(
                main.next_slice()[STATE + 25 * index],
                current + left + right,
            );
        }
    }
}
