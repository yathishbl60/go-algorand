// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::binding::equal;
use super::{CHILD_PRE, COINS, KEY, MlKemEncapsulationAir, RHO, WORK, WORK_WIDTH};
use crate::{Value, mlkem_stream_derivation_air::MlKemStreamDerivationAir};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemEncapsulationAir {
    pub(super) fn eval_derivation_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let flag = builder.preprocessed().current_slice()[CHILD_PRE];
        for (root, child) in pairs(main.current_slice()).expect("fixed derivation wires") {
            equal(builder, flag, root, child);
        }
    }
}

type Pair<'a, T> = (&'a [T], &'a [T]);

fn pairs<T>(row: &[T]) -> Option<[Pair<'_, T>; 5]> {
    let child = &row[WORK..WORK + WORK_WIDTH];
    let air = MlKemStreamDerivationAir;
    Some([
        (&row[2304..3072], air.key_polynomials(child)?),
        (&row[RHO..KEY], air.rho_bits(child)?),
        (&row[7953..8209], air.message_bits(child)?),
        (&row[KEY..COINS], air.shared_secret_bits(child)?),
        (&row[COINS..WORK], air.coins_bits(child)?),
    ])
}
