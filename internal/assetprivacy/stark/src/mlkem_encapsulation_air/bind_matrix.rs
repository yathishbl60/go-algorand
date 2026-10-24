// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, KEY, MlKemEncapsulationAir, RHO, WORK, binding::equal};
use crate::{Value, mlkem_stream_matrix_air::MlKemStreamMatrixAir};
use p3_air::{AirBuilder, BaseAir, WindowAccess};

impl MlKemEncapsulationAir {
    pub(super) fn eval_matrix_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..9 {
            matrix(builder, index);
        }
    }
}

fn matrix<AB: AirBuilder<F = Value>>(builder: &mut AB, index: usize) {
    let main = builder.main();
    let row = main.current_slice();
    let air = MlKemStreamMatrixAir::new((index % 3) as u8, (index / 3) as u8)
        .expect("fixed matrix coordinates");
    let child = &row[WORK..WORK + air.width()];
    let flag = builder.preprocessed().current_slice()[CHILD_PRE + 4 * (1 + index)];
    let seed = air.seed_bits(child).expect("seed");
    let output = air.coefficients(child).expect("coefficients");
    equal(builder, flag, &row[RHO..KEY], seed);
    equal(builder, flag, &row[256 * index..256 * (index + 1)], output);
}
