// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LOCAL_BITS, MlKemEncodingAir, PACKED};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemEncodingAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (column, &value) in row.iter().take(self.scratch()).enumerate() {
            builder
                .when_transition()
                .assert_eq(value, main.next_slice()[column]);
        }
        for &bit in &row[PACKED..self.scratch()] {
            builder.assert_bool(bit);
        }
    }

    pub(super) fn eval_selection<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let selectors = prep.current_slice();
        let input: AB::Expr = (0..256).map(|i| row[i] * selectors[i]).sum();
        builder.assert_eq(row[self.scratch()], input);
        for bit in 0..12 {
            self.eval_bit(builder, bit);
        }
    }

    fn eval_bit<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, bit: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let selected = if bit < self.bits {
            (0..256)
                .map(|i| row[PACKED + i * self.bits + bit] * prep.current_slice()[i])
                .sum()
        } else {
            AB::Expr::ZERO
        };
        builder.assert_eq(row[self.scratch() + LOCAL_BITS + bit], selected);
    }
}
