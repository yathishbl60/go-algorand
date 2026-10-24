// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, EnvelopeAir, OPENING, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl EnvelopeAir {
    pub(super) fn eval_note_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 16];
        for bit in 0..4096 {
            let column = self.note.opening_bit_column(bit / 8, bit % 8);
            builder
                .when(flag)
                .assert_eq(row[OPENING + bit], row[WORK + column]);
        }
    }
}
