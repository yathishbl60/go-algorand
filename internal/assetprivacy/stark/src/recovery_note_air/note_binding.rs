// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, COMMIT, FLAGS, OWNER, RecoveryNoteAir, SERIAL, VALUE, WORK, binding::equal};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl RecoveryNoteAir {
    pub(super) fn eval_note_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + self.note.width()];
        let flag = builder.preprocessed().current_slice()[FLAGS];
        for (root, byte) in [(ACCOUNT..VALUE, 137), (SERIAL..OWNER, 177)] {
            for (bit, &value) in row[root].iter().enumerate() {
                let source = child[self.note.opening_bit_column(byte + bit / 8, bit % 8)];
                builder.when(flag).assert_eq(value, source);
            }
        }
        equal(
            builder,
            flag,
            &row[VALUE..SERIAL],
            &self.note.value_bits(child).expect("note value"),
        );
        self.eval_note_owner(builder);
    }
    fn eval_note_owner<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[FLAGS];
        for limb in 0..COMMIT - OWNER {
            let expected = (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
                let column = WORK
                    + self
                        .note
                        .opening_bit_column(73 + limb * 2 + bit / 8, bit % 8);
                value.double() + row[column]
            });
            builder.when(flag).assert_eq(row[OWNER + limb], expected);
        }
    }
}
