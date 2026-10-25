// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, CREATED, EPOCH, FLAGS, LEAF, OWNER, RHO, WORK, binding::equal};
use crate::{
    Value,
    view_key_air::{ViewKeyAir, WIDTH, wires},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

pub(super) fn eval<AB: AirBuilder<F = Value>>(air: &ViewKeyAir, builder: &mut AB) {
    let main = builder.main();
    let row = main.current_slice();
    let child = &row[WORK..WORK + WIDTH];
    let key = wires(child).expect("view key wires");
    let flag = builder.preprocessed().current_slice()[FLAGS];
    for (root, source) in [
        (&row[ACCOUNT..EPOCH], key.account_bits),
        (&row[EPOCH..CREATED], key.epoch_bits),
        (&row[CREATED..RHO], key.created_bits),
        (&row[RHO..OWNER], key.rho_bits),
        (&row[LEAF..WORK], key.commitment_limbs),
    ] {
        equal(builder, flag, root, source);
    }
    coefficients(air, builder);
}

fn coefficients<AB: AirBuilder<F = Value>>(air: &ViewKeyAir, builder: &mut AB) {
    let main = builder.main();
    let row = main.current_slice();
    let child = &row[WORK..WORK + WIDTH];
    let flag = builder.preprocessed().current_slice()[FLAGS];
    for (index, &value) in row[..ACCOUNT].iter().enumerate() {
        let bits = air.coefficient_bits(child, index).expect("coefficient");
        let packed = bits
            .iter()
            .rev()
            .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit);
        builder.when(flag).assert_eq(value, packed);
    }
}
