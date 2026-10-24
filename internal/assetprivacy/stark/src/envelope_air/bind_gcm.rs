// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, CHILD_PRE, COMMIT, EnvelopeAir, KEY, OPENING, WORK, binding::equal};
use crate::{
    Value,
    gcm_vertical::{WIDTH, wires},
};
use p3_air::{AirBuilder, WindowAccess};

impl EnvelopeAir {
    pub(super) fn eval_gcm_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = wires(&row[WORK..WORK + WIDTH]).expect("GCM wires");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 12];
        equal(builder, flag, &row[AAD..KEY], child.associated);
        equal(builder, flag, &row[KEY..OPENING], child.key);
        equal(builder, flag, &row[OPENING..COMMIT], child.plaintext);
    }
}
