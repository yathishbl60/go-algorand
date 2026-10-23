// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MembershipAir {
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, level: usize) {
        let main = builder.main();
        let offset = self.stage_start(level) + NUM_KECCAK_COLS;
        for (byte, value) in self.prefix.iter().copied().chain([level as u8]).enumerate() {
            for bit in 0..8 {
                builder.assert_eq(
                    main.current_slice()[offset + byte * 8 + bit],
                    Value::from_bool(value & (1 << bit) != 0),
                );
            }
        }
    }

    pub(super) fn eval_stage<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, level: usize) {
        let main = builder.main();
        let direction = main.current_slice()[self.direction_column(level)];
        builder.assert_bool(direction);
        builder
            .when_transition()
            .assert_eq(direction, main.next_slice()[self.direction_column(level)]);
        for limb in 0..32 {
            self.eval_child(builder, level, limb);
            builder.when_transition().assert_eq(
                main.current_slice()[self.digest_start(level) + limb],
                main.next_slice()[self.digest_start(level) + limb],
            );
        }
    }

    pub(super) fn eval_root<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for limb in 0..32 {
            builder.assert_eq(
                main.current_slice()[self.digest_start(DEPTH - 1) + limb],
                builder.public_values()[limb],
            );
        }
    }
}
