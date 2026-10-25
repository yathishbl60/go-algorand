// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CANONICAL, ViewKeyAir};
use crate::{
    Value,
    mlkem_field::{Canonical, eval_canonical},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl ViewKeyAir {
    pub(super) fn eval_canonical<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let pre = builder.preprocessed().current_slice();
        let selected: AB::Expr = (0..768)
            .map(|index| pre[index] * self.coefficient::<AB>(row, index))
            .sum();
        let value = Canonical {
            value: row[CANONICAL],
            bits: row[CANONICAL + 1..CANONICAL + 13].try_into().unwrap(),
            gap: row[CANONICAL + 13..CANONICAL + 25].try_into().unwrap(),
        };
        eval_canonical(builder, &value);
        builder.assert_eq(value.value, selected);
    }
    fn coefficient<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var], index: usize) -> AB::Expr {
        self.coefficient_bits(row, index)
            .expect("fixed coefficient")
            .iter()
            .rev()
            .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit)
    }
}
