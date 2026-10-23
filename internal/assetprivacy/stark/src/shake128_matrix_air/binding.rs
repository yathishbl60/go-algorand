// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SEED, Shake128MatrixAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl Shake128MatrixAir {
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (column, &value) in row.iter().enumerate().skip(SEED) {
            builder.assert_bool(value);
            builder
                .when_transition()
                .assert_eq(row[column], main.next_slice()[column]);
        }
    }
}
