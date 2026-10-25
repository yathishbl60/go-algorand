// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::record::RecordAir;
use super::{ACCOUNT, FLAGS, LEAF, OWNER, RHO, RegisteredKeyAir, WORK};
use crate::Value;
use p3_air::{AirBuilder, BaseAir, WindowAccess};

impl RegisteredKeyAir {
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
        self.eval_unused(builder);
    }
    fn eval_unused<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let unused = match self.record {
            RecordAir::View(_) => OWNER..LEAF,
            RecordAir::Spend(_) => 0..ACCOUNT,
        };
        for &value in &main.current_slice()[unused] {
            builder.assert_zero(value);
        }
        if matches!(self.record, RecordAir::Spend(_)) {
            for &value in &main.current_slice()[RHO..OWNER] {
                builder.assert_zero(value);
            }
        }
    }
    pub(super) fn eval_record_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        match &self.record {
            RecordAir::View(air) => super::bind_view::eval(air, builder),
            RecordAir::Spend(air) => super::bind_spend::eval(air, builder),
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
            self.membership.leaf_limbs(child).expect("leaf"),
        );
    }
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "registered key binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
