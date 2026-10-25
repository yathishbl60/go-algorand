// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DIGEST, DIR, FLAGS, LEAF, MembershipVerticalAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MembershipVerticalAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let transition = builder.preprocessed().current_slice()[FLAGS + 3];
        builder.assert_bool(row[DIR]);
        for (column, &leaf) in row.iter().enumerate().skip(LEAF).take(32) {
            builder
                .when_transition()
                .assert_eq(leaf, main.next_slice()[column]);
        }
        for column in (DIGEST..DIGEST + 32).chain(core::iter::once(DIR)) {
            builder
                .when(transition)
                .assert_eq(row[column], main.next_slice()[column]);
        }
    }
}
