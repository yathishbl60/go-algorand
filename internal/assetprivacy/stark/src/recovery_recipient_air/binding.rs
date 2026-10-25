// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCOUNT, CHILD_PRE, OWNER, RHO, RecoveryRecipientAir, SPEND_EPOCH, VALUE, VIEW_EPOCH, WORK,
};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl RecoveryRecipientAir {
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
    pub(super) fn eval_envelope_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + crate::envelope_air::WIDTH];
        let key = crate::envelope_air::wires(child).expect("envelope");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE];
        for (root, source) in [
            (&row[..RHO], key.public_polynomials),
            (&row[RHO..VIEW_EPOCH], key.rho_bits),
            (&row[VIEW_EPOCH..ACCOUNT], key.epoch_bits),
            (&row[ACCOUNT..OWNER], &key.opening_bits[137 * 8..169 * 8]),
            (&row[OWNER..SPEND_EPOCH], &key.opening_bits[73 * 8..137 * 8]),
        ] {
            equal(builder, flag, root, source);
        }
        let value = self.envelope.value_bits(child).expect("note amount");
        equal(builder, flag, &row[VALUE..WORK], &value);
    }
    pub(super) fn eval_view_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let key =
            crate::registered_key_air::wires(&row[WORK..WORK + crate::registered_key_air::WIDTH])
                .expect("view");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 4];
        for (root, source) in [
            (&row[..RHO], key.public_polynomials),
            (&row[RHO..VIEW_EPOCH], key.rho_bits),
            (&row[VIEW_EPOCH..ACCOUNT], key.epoch_bits),
            (&row[ACCOUNT..OWNER], key.account_bits),
        ] {
            equal(builder, flag, root, source);
        }
    }
    pub(super) fn eval_spend_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let key =
            crate::registered_key_air::wires(&row[WORK..WORK + crate::registered_key_air::WIDTH])
                .expect("spend");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 8];
        for (root, source) in [
            (&row[SPEND_EPOCH..VALUE], key.epoch_bits),
            (&row[ACCOUNT..OWNER], key.account_bits),
            (&row[OWNER..SPEND_EPOCH], key.owner_bits),
        ] {
            equal(builder, flag, root, source);
        }
    }
}

fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "recipient binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
