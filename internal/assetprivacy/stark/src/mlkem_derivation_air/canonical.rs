// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemDerivationAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MlKemDerivationAir {
    pub(super) fn eval_canonical<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for bit in 0..12 {
            self.eval_key_bit(builder, bit);
        }
        let main = builder.main();
        let row = &main.current_slice()[self.scratch()..];
        for &bit in row {
            builder.assert_bool(bit);
        }
        let decoded = |start: usize| {
            (0..12)
                .rev()
                .fold(AB::Expr::ZERO, |v, b| v.double() + row[start + b])
        };
        builder.assert_eq(decoded(0) + decoded(12), Value::from_u16(3328));
    }

    fn eval_key_bit<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, bit: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let selected: AB::Expr = (0..COEFFICIENTS)
            .map(|i| prep.current_slice()[i] * row[NUM_KECCAK_COLS + i * 12 + bit])
            .sum();
        builder.assert_eq(row[self.scratch() + bit], selected);
    }
}
