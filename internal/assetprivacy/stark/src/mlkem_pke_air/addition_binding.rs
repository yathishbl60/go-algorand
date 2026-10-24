// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ADD_INDEX, ADD_SLOT, FINAL, MESSAGE, MlKemPkeAir, NOISE, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemPkeAir {
    pub(super) fn eval_addition_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let flags = prep.current_slice();
        let active: AB::Expr = flags[ADD_SLOT..ADD_SLOT + 4]
            .iter()
            .map(|&v| v.into())
            .sum();
        let raw: AB::Expr = (0..256)
            .map(|i| row[WORK + 512 + i] * flags[ADD_INDEX + i])
            .sum();
        let message: AB::Expr = (0..256)
            .map(|i| row[MESSAGE + i] * flags[ADD_INDEX + i])
            .sum();
        let mu = message * flags[ADD_SLOT + 3] * Value::from_u16(1665);
        builder.when(active.clone()).assert_eq(row[WORK], raw);
        builder.when(active).assert_eq(row[WORK + 76], mu);
    }

    pub(super) fn eval_error_slots<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for slot in 0..4 {
            self.eval_error_slot(builder, slot);
        }
    }

    fn eval_error_slot<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, slot: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let flags = prep.current_slice();
        let flag = flags[ADD_SLOT + slot];
        let error: AB::Expr = (0..256)
            .map(|i| row[NOISE + (3 + slot) * 256 + i] * flags[ADD_INDEX + i])
            .sum();
        let output: AB::Expr = (0..256)
            .map(|i| row[FINAL + slot * 256 + i] * flags[ADD_INDEX + i])
            .sum();
        builder.when(flag).assert_eq(row[WORK + 25], error);
        builder.when(flag).assert_eq(row[WORK + 101], output);
    }
}
