// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, CHILD_PRE, EPOCH, EnvelopeAir, KEY, NONZERO, RHO, SHARED, WORK, binding::equal};
use crate::Value;
use p3_air::{AirBuilder, BaseAir, WindowAccess};

impl EnvelopeAir {
    pub(super) fn eval_aad_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + self.aad.width()];
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 4];
        let t = self.aad.key_polynomials(child).expect("key coefficients");
        let rho = self.aad.rho_bits(child).expect("key seed");
        let aad = self.aad.associated_bits(child).expect("associated data");
        equal(builder, flag, &row[..RHO], t);
        equal(builder, flag, &row[RHO..SHARED], rho);
        equal(builder, flag, &row[AAD..KEY], aad);
        let epoch = self.aad.epoch_bits(child).expect("private epoch");
        equal(builder, flag, &row[EPOCH..NONZERO], epoch);
    }
}
