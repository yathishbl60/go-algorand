// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemNoiseAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemNoiseAir {
    pub(super) fn eval_coefficient<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, i: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let start = self.flag_start() + 5 * i;
        let flags: [AB::Var; 5] = row[start..start + 5].try_into().expect("five CBD flags");
        builder.assert_bools(flags);
        let count: AB::Expr = flags.iter().copied().map(Into::into).sum();
        builder.assert_one(count);
        let signed = flags[3] + flags[4] * Value::TWO - flags[1] - flags[0] * Value::TWO;
        let start = self.bit_start() + 4 * i;
        builder.assert_eq(
            signed,
            row[start] + row[start + 1] - row[start + 2] - row[start + 3],
        );
        let residue = flags[0] * Value::from_u16(3327)
            + flags[1] * Value::from_u16(3328)
            + flags[3]
            + flags[4] * Value::TWO;
        builder.assert_eq(row[self.coefficient_start() + i], residue);
    }
}
