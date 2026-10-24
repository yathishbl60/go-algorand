// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ADD_SLOT, MlKemPkeAir, WORK};
use crate::{
    Value,
    mlkem_field::{Add, eval_add},
    mlkem_product_air::canonical,
};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemPkeAir {
    pub(super) fn eval_additions<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_addition_bindings(builder);
        self.eval_error_slots(builder);
        let main = builder.main();
        let row = main.current_slice();
        let gate: AB::Expr = builder.preprocessed().current_slice()[ADD_SLOT..ADD_SLOT + 4]
            .iter()
            .map(|&v| v.into())
            .sum();
        for (left, right, output, carry) in [(0, 25, 50, 75), (50, 76, 101, 126)] {
            let value = Add {
                left: canonical(row, WORK + left),
                right: canonical(row, WORK + right),
                output: canonical(row, WORK + output),
                carry: row[WORK + carry],
            };
            eval_add(&mut builder.when(gate.clone()), &value);
        }
    }
}
