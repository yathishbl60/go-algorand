// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, CREATED, EPOCH, FLAGS, LEAF, OWNER, RHO, WORK, binding::equal};
use crate::{
    Value,
    spend_key_air::{SpendKeyAir, WIDTH, wires},
};
use p3_air::{AirBuilder, WindowAccess};

pub(super) fn eval<AB: AirBuilder<F = Value>>(_: &SpendKeyAir, builder: &mut AB) {
    let main = builder.main();
    let row = main.current_slice();
    let key = wires(&row[WORK..WORK + WIDTH]).expect("spend key wires");
    let flag = builder.preprocessed().current_slice()[FLAGS];
    for (root, source) in [
        (&row[ACCOUNT..EPOCH], key.account_bits),
        (&row[EPOCH..CREATED], key.epoch_bits),
        (&row[CREATED..RHO], key.created_bits),
        (&row[OWNER..LEAF], key.owner_bits),
        (&row[LEAF..WORK], key.commitment_limbs),
    ] {
        equal(builder, flag, root, source);
    }
}
