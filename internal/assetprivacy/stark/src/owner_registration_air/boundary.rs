// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnerRegistrationAir, WORK};
use crate::{Value, serial_keccak::STATE_BITS};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl OwnerRegistrationAir {
    pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for bit in 0..STATE_BITS {
            let byte = if bit < 72 * 8 {
                self.prefix[bit / 8]
            } else {
                0
            };
            let expected = Value::from_bool(byte & (1 << (bit % 8)) != 0);
            builder
                .when_first_row()
                .assert_eq(main.current_slice()[WORK + bit], expected);
        }
    }

    pub(super) fn eval_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for limb in 0..32 {
            let value = (0..16).rev().fold(AB::Expr::ZERO, |acc, bit| {
                acc.double() + main.current_slice()[WORK + limb * 16 + bit]
            });
            let expected = builder.public_values()[limb];
            builder.when_last_row().assert_eq(value, expected);
        }
    }
}
