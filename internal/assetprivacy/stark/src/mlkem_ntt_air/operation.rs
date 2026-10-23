// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BORROW, CARRY, HIGH, LEFT, LOW, MlKemNttAir, PRODUCT, QUOTIENT, RIGHT, SCALE, ZETA};
use crate::{Value, mlkem_field::Q};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemNttAir {
    pub(super) fn eval_operation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        if self.inverse {
            self.eval_inverse(builder);
            self.eval_scale(builder);
        } else {
            self.eval_forward(builder);
        }
    }

    fn eval_forward<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let r = main.current_slice();
        let zeta = builder.preprocessed().current_slice()[ZETA];
        let q = Value::from_u16(Q);
        builder.assert_eq(zeta * r[RIGHT], r[QUOTIENT] * q + r[PRODUCT]);
        builder.assert_eq(r[LEFT] + r[PRODUCT], r[CARRY] * q + r[LOW]);
        builder.assert_eq(r[LEFT] + r[BORROW] * q, r[PRODUCT] + r[HIGH]);
    }

    fn eval_inverse<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let r = main.current_slice();
        let prep = builder.preprocessed();
        let zeta = prep.current_slice()[ZETA];
        let gate = AB::Expr::ONE - prep.current_slice()[SCALE];
        let q = Value::from_u16(Q);
        builder
            .when(gate.clone())
            .assert_eq(r[LEFT] + r[RIGHT], r[CARRY] * q + r[LOW]);
        builder
            .when(gate.clone())
            .assert_eq(r[RIGHT] + r[BORROW] * q, r[LEFT] + r[PRODUCT]);
        builder
            .when(gate)
            .assert_eq(zeta * r[PRODUCT], r[QUOTIENT] * q + r[HIGH]);
    }

    fn eval_scale<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let r = main.current_slice();
        let prep = builder.preprocessed();
        let flag = prep.current_slice()[SCALE];
        let product = prep.current_slice()[ZETA] * r[LEFT];
        builder
            .when(flag)
            .assert_eq(product, r[QUOTIENT] * Value::from_u16(Q) + r[LOW]);
        for column in [PRODUCT, HIGH, CARRY, BORROW] {
            builder.when(flag).assert_zero(r[column]);
        }
    }
}
