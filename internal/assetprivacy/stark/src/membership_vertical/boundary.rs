// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CURRENT, DIGEST, FLAGS, LEAF, MembershipVerticalAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MembershipVerticalAir {
    pub(super) fn eval_boundaries<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let last = builder.preprocessed().current_slice()[FLAGS + 2];
        for limb in 0..32 {
            builder
                .when_first_row()
                .assert_eq(row[CURRENT + limb], row[LEAF + limb]);
            builder.when_transition().assert_eq(
                main.next_slice()[CURRENT + limb],
                row[CURRENT + limb] + last * (row[DIGEST + limb] - row[CURRENT + limb]),
            );
            let public = builder.public_values()[limb];
            builder
                .when_last_row()
                .assert_eq(row[CURRENT + limb], public);
        }
    }
}
