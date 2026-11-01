// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnerRegistrationAir, WORK};
use crate::{Value, serial_keccak::STATE_BITS};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl OwnerRegistrationAir {
    pub(super) fn eval_group<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (local, next) in row[..WORK].iter().zip(&main.next_slice()[..WORK]) {
            builder.when_transition().assert_eq(*local, *next);
        }
        for limb in 0..32 {
            builder.assert_eq(row[1 + limb], builder.public_values()[32 + limb]);
        }
    }

    pub(super) fn eval_hold<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let pre = builder.preprocessed();
        let active: AB::Expr = pre.current_slice()[..4].iter().map(|&v| v.into()).sum();
        let idle = AB::Expr::ONE - active;
        for bit in 0..STATE_BITS {
            builder.when_transition().when(idle.clone()).assert_eq(
                main.current_slice()[WORK + bit],
                main.next_slice()[WORK + bit],
            );
        }
    }
}
