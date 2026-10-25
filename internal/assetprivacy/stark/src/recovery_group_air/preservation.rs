// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, FLAGS, RecoveryGroupAir, SOURCE_ACCOUNT, SOURCE_OWNER, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl RecoveryGroupAir {
    pub(super) fn eval_preservation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let Some(index) = self.operation.remainder_index() else {
            return;
        };
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS + (index + 1) * 4];
        for bit in 0..SOURCE_OWNER - SOURCE_ACCOUNT {
            let column = WORK + self.output.opening_bit_column(137 + bit / 8, bit % 8);
            builder
                .when(flag)
                .assert_eq(row[SOURCE_ACCOUNT + bit], row[column]);
        }
        self.eval_preserved_owner(builder, flag);
    }
    fn eval_preserved_owner<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, flag: AB::Var) {
        let main = builder.main();
        let row = main.current_slice();
        let owner = core::array::from_fn::<_, 32, _>(|limb| {
            (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
                let column = WORK
                    + self
                        .output
                        .opening_bit_column(73 + limb * 2 + bit / 8, bit % 8);
                value.double() + row[column]
            })
        });
        for (&root, value) in row[SOURCE_OWNER..CARRY].iter().zip(owner) {
            builder.when(flag).assert_eq(root, value);
        }
    }
}
