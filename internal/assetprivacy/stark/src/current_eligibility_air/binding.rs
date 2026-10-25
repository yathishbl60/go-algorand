// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, CurrentEligibilityAir, FLAGS, LEAF, REVISION, UPDATED, WORK};
use crate::Value;
use p3_air::{AirBuilder, BaseAir, WindowAccess};

impl CurrentEligibilityAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        for &bit in &main.current_slice()[ACCOUNT..LEAF] {
            builder.assert_bool(bit);
        }
    }
    pub(super) fn eval_record_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = crate::eligible_record_air::wires(&row[WORK..WORK + self.record.width()])
            .expect("eligible record");
        let flag = builder.preprocessed().current_slice()[FLAGS];
        for (root, source) in [
            (&row[ACCOUNT..REVISION], child.account_bits),
            (&row[REVISION..UPDATED], child.revision_bits),
            (&row[UPDATED..LEAF], child.updated_bits),
            (&row[LEAF..WORK], child.commitment_limbs),
        ] {
            equal(builder, flag, root, source);
        }
    }
    pub(super) fn eval_membership_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + self.membership.width()];
        let flag = builder.preprocessed().current_slice()[FLAGS + 4];
        equal(
            builder,
            flag,
            &row[LEAF..WORK],
            self.membership.leaf_limbs(child).expect("eligibility leaf"),
        );
    }
}

fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "eligibility binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
