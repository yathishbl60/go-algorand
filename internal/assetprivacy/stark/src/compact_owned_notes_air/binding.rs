// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnedNotesAir, schema::*};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeField64;

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn eval_bridge<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let (row, next) = (main.current_slice(), main.next_slice());
        let hold = builder.periodic_values()[HOLD_OWNER];
        let dummy = builder.periodic_values()[4];
        for column in OWNER..OWNER + 32 {
            builder
                .when_transition()
                .when(hold)
                .assert_eq(next[column], row[column]);
            builder.when(dummy).assert_zero(row[column]);
        }
        builder.assert_eq(row[SELECTED], selected_owner::<AB>(builder, row));
    }
}

fn selected_owner<AB: AirBuilder>(builder: &AB, row: &[AB::Var]) -> AB::Expr {
    (0..32)
        .map(|i| {
            let flag: AB::Expr = builder.periodic_values()[SELECT + i].into();
            flag * row[OWNER + i]
        })
        .sum()
}
