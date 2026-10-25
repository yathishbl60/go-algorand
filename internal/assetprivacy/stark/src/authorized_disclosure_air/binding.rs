// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AuthorizedDisclosureAir, CHILD_PRE, EPOCH, RHO, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl AuthorizedDisclosureAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            let value = main.current_slice()[column];
            builder
                .when_transition()
                .assert_eq(value, main.next_slice()[column]);
            if column >= RHO {
                builder.assert_bool(value);
            }
        }
    }
    pub(super) fn eval_authority<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for (index, &byte) in self.authority.iter().enumerate() {
            let public = builder.public_values()[1100 + index];
            builder.assert_eq(public, Value::from_u8(byte));
        }
    }
    pub(super) fn eval_envelope_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = crate::envelope_air::wires(&row[WORK..WORK + crate::envelope_air::WIDTH])
            .expect("envelope wires");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE];
        for (root, source) in [
            (&row[..RHO], child.public_polynomials),
            (&row[RHO..EPOCH], child.rho_bits),
            (&row[EPOCH..WORK], child.epoch_bits),
        ] {
            equal(builder, flag, root, source);
        }
    }
    pub(super) fn eval_view_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child =
            crate::registered_key_air::wires(&row[WORK..WORK + crate::registered_key_air::WIDTH])
                .expect("registered view wires");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 4];
        for (root, source) in [
            (&row[..RHO], child.public_polynomials),
            (&row[RHO..EPOCH], child.rho_bits),
            (&row[EPOCH..WORK], child.epoch_bits),
        ] {
            equal(builder, flag, root, source);
        }
        for (bit, &value) in child.account_bits.iter().enumerate() {
            let expected = Value::from_bool(self.authority[bit / 8] & (1 << (bit % 8)) != 0);
            builder.when(flag).assert_eq(value, expected);
        }
    }
}

fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "disclosure binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
