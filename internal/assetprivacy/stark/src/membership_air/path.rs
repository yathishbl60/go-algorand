// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MembershipAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MembershipAir {
    pub(super) fn eval_child<AB: AirBuilder<F = Value>>(
        &self,
        builder: &mut AB,
        level: usize,
        limb: usize,
    ) {
        let main = builder.main();
        let row = main.current_slice();
        let direction = row[self.direction_column(level)];
        let prior = if level == 0 {
            self.leaf_start()
        } else {
            self.digest_start(level - 1)
        };
        let left = self.child_limb::<AB>(row, level, 0, limb);
        let right = self.child_limb::<AB>(row, level, 1, limb);
        builder.assert_eq(row[prior + limb], left.clone() + direction * (right - left));
    }

    fn child_limb<AB: AirBuilder<F = Value>>(
        &self,
        row: &[AB::Var],
        level: usize,
        child: usize,
        limb: usize,
    ) -> AB::Expr {
        let start = self.stage_start(level) + NUM_KECCAK_COLS + (self.prefix.len() + 1) * 8;
        let offset = start + child * 512 + limb * 16;
        (0..16)
            .rev()
            .fold(AB::Expr::ZERO, |v, bit| v.double() + row[offset + bit])
    }
}
