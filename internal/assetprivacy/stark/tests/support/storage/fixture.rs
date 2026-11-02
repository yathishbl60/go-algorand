// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::V;
use p3_air::{Air, AirBuilder, BaseAir, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use std::borrow::Cow;

pub struct Fixture {
    pub periodic: Vec<Vec<V>>,
}

impl BaseAir<V> for Fixture {
    fn width(&self) -> usize {
        2
    }
    fn num_periodic_columns(&self) -> usize {
        self.periodic.len()
    }
    fn periodic_columns(&self) -> Cow<'_, [Vec<V>]> {
        Cow::Borrowed(&self.periodic)
    }
}

impl<AB: AirBuilder<F = V>> Air<AB> for Fixture {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let next = main.next_slice();
        let periodic: AB::Expr = builder.periodic_values()[0].into();
        builder.when_first_row().assert_eq(row[0], V::from_u8(19));
        builder.when_last_row().assert_eq(row[1], V::from_u8(7));
        builder
            .when_transition()
            .assert_eq(next[0], row[1] + periodic.clone());
        builder.assert_zero(periodic * (row[0] * row[0] - row[1]));
    }
}
