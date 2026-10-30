// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, GROUP, ShieldGroupAir, WORK};
use crate::{
    Value,
    amounts::{self, Amounts},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl ShieldGroupAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        for &bit in &main.current_slice()[..128] {
            builder.assert_zero(bit);
        }
    }
    pub(super) fn eval_amounts<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let values = core::array::from_fn(|index| {
            row[index * 64..index * 64 + 64]
                .try_into()
                .expect("amount width")
        });
        let carry = row[CARRY..GROUP].try_into().expect("carry width");
        amounts::eval(builder, &Amounts { values, carry }, self.conversion());
    }
    pub(super) fn eval_public<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for limb in 0..4 {
            let expected = Value::from_u16((self.debit >> (limb * 16)) as u16);
            builder.assert_eq(builder.public_values()[64 + limb], expected);
        }
        let main = builder.main();
        for limb in 0..32 {
            builder.assert_eq(
                main.current_slice()[GROUP + limb],
                builder.public_values()[68 + limb],
            );
        }
    }
}
