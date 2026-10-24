// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, COINS, MlKemEncapsulationAir, WORK, binding::equal};
use crate::{Value, mlkem_noise_air::MlKemNoiseAir};
use p3_air::{AirBuilder, BaseAir, WindowAccess};

impl MlKemEncapsulationAir {
    pub(super) fn eval_noise_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..7 {
            noise(builder, index);
        }
    }
}

fn noise<AB: AirBuilder<F = Value>>(builder: &mut AB, index: usize) {
    let main = builder.main();
    let row = main.current_slice();
    let air = MlKemNoiseAir::new(index as u8).expect("fixed noise nonce");
    let child = &row[WORK..WORK + air.width()];
    let flag = builder.preprocessed().current_slice()[CHILD_PRE + 4 * (10 + index)];
    for bit in 0..256 {
        let column = air.seed_bit_column(bit / 8, bit % 8).expect("seed");
        builder
            .when(flag)
            .assert_eq(row[COINS + bit], child[column]);
    }
    let output = air.coefficients(child).expect("coefficients");
    equal(
        builder,
        flag,
        &row[6161 + 256 * index..6161 + 256 * (index + 1)],
        output,
    );
}
