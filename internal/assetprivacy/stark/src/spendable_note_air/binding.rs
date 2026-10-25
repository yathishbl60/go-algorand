// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCOUNT, COMMIT, FLAGS, INVERSE, OWNER, SECRET, SERIAL, SpendableNoteAir, VALUE, WORK,
};
use crate::Value;
use p3_air::{AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl SpendableNoteAir {
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
        let sum: AB::Expr = main.current_slice()[SECRET..OWNER]
            .iter()
            .copied()
            .map(Into::into)
            .sum();
        builder.assert_one(sum * main.current_slice()[INVERSE]);
    }
    pub(super) fn eval_owner_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let flag = builder.preprocessed().current_slice()[FLAGS + 4];
        prefix(builder, flag, &self.owner_prefix);
        let row = main.current_slice();
        let start = WORK + NUM_KECCAK_COLS + self.owner_prefix.len() * 8;
        equal(builder, flag, &row[SECRET..OWNER], &row[start..start + 256]);
    }
    pub(super) fn eval_nullifier_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let flag = builder.preprocessed().current_slice()[FLAGS + 8];
        prefix(builder, flag, &self.nullifier_prefix);
        let row = main.current_slice();
        let start = WORK + NUM_KECCAK_COLS + self.nullifier_prefix.len() * 8;
        equal(
            builder,
            flag,
            &row[SERIAL..SECRET],
            &row[start..start + 256],
        );
    }
    pub(super) fn eval_membership_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS + 12];
        let child = &row[WORK..WORK + self.membership.width()];
        equal(
            builder,
            flag,
            &row[COMMIT..INVERSE],
            self.membership.leaf_limbs(child).expect("note leaf"),
        );
    }
    pub(super) fn eval_eligibility_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS + 16];
        let child =
            crate::current_eligibility_air::wires(&row[WORK..WORK + self.eligibility.width()])
                .expect("eligibility wires");
        equal(builder, flag, &row[ACCOUNT..VALUE], child.account_bits);
    }
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "spend note binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}

fn prefix<AB: AirBuilder<F = Value>>(builder: &mut AB, flag: AB::Var, bytes: &[u8]) {
    let main = builder.main();
    for (index, &byte) in bytes.iter().enumerate() {
        for bit in 0..8 {
            let expected = Value::from_bool(byte & (1 << bit) != 0);
            builder.when(flag).assert_eq(
                main.current_slice()[WORK + NUM_KECCAK_COLS + index * 8 + bit],
                expected,
            );
        }
    }
}
