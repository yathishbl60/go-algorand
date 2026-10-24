// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CANDIDATE, GAP, MlKemStreamMatrixAir, RATE, REJECT, binding::bits};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemStreamMatrixAir {
    pub(super) fn eval_candidate<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let schedule = builder.preprocessed().current_slice();
        for bit in 0..12 {
            let selected: AB::Expr = (0..112).map(|index| schedule[index] * row[RATE + 12 * index + bit]).sum();
            builder.assert_bool(row[CANDIDATE + bit]);
            builder.assert_eq(row[CANDIDATE + bit], selected);
        }
    }
    pub(super) fn eval_comparison<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[GAP..REJECT] { builder.assert_bool(bit); }
        builder.assert_bool(row[REJECT]);
        builder.assert_eq(bits::<AB>(&row[CANDIDATE..GAP]) + Value::from_u16(767), row[REJECT] * Value::from_u16(4096) + bits::<AB>(&row[GAP..REJECT]));
    }
}
