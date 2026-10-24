// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    A0, A1, B0, B1, C0, C1, CARRY0, CARRY1, GAMMA, GP11, MlKemProductAir, P00, P01, P10, P11, view,
};
use crate::{
    Value,
    mlkem_field::{Add, eval_add, eval_mul},
};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemProductAir {
    // The parent binds the source operands and gamma before reusing this arithmetic.
    pub(crate) fn eval_arithmetic<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (start, left, right) in [
            (P11, A1, B1),
            (GP11, P11, GAMMA),
            (P00, A0, B0),
            (P01, A0, B1),
            (P10, A1, B0),
        ] {
            eval_mul(builder, &view::multiply(row, start, left, right));
        }
        self.eval_sums(builder);
    }

    fn eval_sums<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (left, right, output, carry) in [(P00, GP11, C0, CARRY0), (P01, P10, C1, CARRY1)] {
            let value = Add {
                left: view::canonical(row, left),
                right: view::canonical(row, right),
                output: view::canonical(row, output),
                carry: row[carry],
            };
            eval_add(builder, &value);
        }
    }
}
