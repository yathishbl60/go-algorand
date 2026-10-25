// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CURRENT, DIR, FLAGS, MembershipVerticalAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MembershipVerticalAir {
    pub(super) fn eval_child<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[FLAGS];
        for limb in 0..32 {
            let left = child_limb::<AB>(row, 0, limb);
            let right = child_limb::<AB>(row, 1, limb);
            builder.when(active).assert_eq(
                row[CURRENT + limb],
                left.clone() + row[DIR] * (right - left),
            );
        }
    }
}

fn child_limb<AB: AirBuilder<F = Value>>(row: &[AB::Var], child: usize, limb: usize) -> AB::Expr {
    let start = NUM_KECCAK_COLS + 149 * 8 + child * 512 + limb * 16;
    row[start..start + 16]
        .iter()
        .rev()
        .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit)
}
