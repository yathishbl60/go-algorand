// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, COMMIT, FLAGS, OWNER, RecoveryNoteAir, SERIAL, WORK};
use crate::Value;
use p3_air::{AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl RecoveryNoteAir {
    pub(super) fn eval_roots<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        for &value in &main.current_slice()[ACCOUNT..OWNER] {
            builder.assert_bool(value);
        }
    }
    pub(super) fn eval_nullifier_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let flag = builder.preprocessed().current_slice()[FLAGS + 4];
        for (index, &byte) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                let expected = Value::from_bool(byte & (1 << bit) != 0);
                builder.when(flag).assert_eq(
                    main.current_slice()[WORK + NUM_KECCAK_COLS + index * 8 + bit],
                    expected,
                );
            }
        }
        let row = main.current_slice();
        let start = WORK + NUM_KECCAK_COLS + self.prefix.len() * 8;
        equal(builder, flag, &row[SERIAL..OWNER], &row[start..start + 256]);
    }
    pub(super) fn eval_membership_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS + 8];
        let child = &row[WORK..WORK + self.membership.width()];
        equal(
            builder,
            flag,
            &row[COMMIT..WORK],
            self.membership.leaf_limbs(child).expect("recovery leaf"),
        );
    }
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "recovery note binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
