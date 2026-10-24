// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, CHILD_PRE, EnvelopeAir, RHO, SHARED, WORK, binding::equal};
use crate::{
    Value,
    mlkem_encapsulation_air::{WIDTH, wires},
};
use p3_air::{AirBuilder, WindowAccess};

impl EnvelopeAir {
    pub(super) fn eval_kem_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = wires(&row[WORK..WORK + WIDTH]).expect("KEM wires");
        let flag = builder.preprocessed().current_slice()[CHILD_PRE];
        equal(builder, flag, &row[..RHO], child.public_polynomials);
        equal(builder, flag, &row[RHO..SHARED], child.rho_bits);
        equal(builder, flag, &row[SHARED..AAD], child.shared_secret_bits);
    }
}
