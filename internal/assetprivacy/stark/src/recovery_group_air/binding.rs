// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, FLAGS, RecoveryGroupAir, SOURCE_ACCOUNT, SOURCE_OWNER, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl RecoveryGroupAir {
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS];
        let child =
            crate::recovery_note_air::wires(&row[WORK..WORK + crate::recovery_note_air::WIDTH])
                .expect("recovery input");
        equal(builder, flag, &row[..64], child.value_bits);
        equal(
            builder,
            flag,
            &row[SOURCE_ACCOUNT..SOURCE_OWNER],
            child.account_bits,
        );
        equal(builder, flag, &row[SOURCE_OWNER..CARRY], child.owner_limbs);
        for index in 0..self.operation.output_count() {
            self.eval_output_value(builder, index);
        }
    }
    fn eval_output_value<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let value = self
            .output
            .value_bits(&row[WORK..WORK + self.output.width()])
            .expect("output width");
        let flag = builder.preprocessed().current_slice()[FLAGS + (index + 1) * 4];
        equal(
            builder,
            flag,
            &bits(row, self.operation.amount_slot(index)),
            &value,
        );
    }
}

pub(super) fn bits<T: Copy>(row: &[T], index: usize) -> [T; 64] {
    row[index * 64..index * 64 + 64]
        .try_into()
        .expect("amount width")
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "recovery group binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
