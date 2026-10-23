// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Canonical, Q, eval_canonical};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub struct Mul<T> {
    pub left: Canonical<T>,
    pub right: Canonical<T>,
    pub output: Canonical<T>,
    pub quotient: Canonical<T>,
}

pub fn eval_mul<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Mul<AB::Var>) {
    eval_canonical(builder, &value.left);
    eval_canonical(builder, &value.right);
    eval_canonical(builder, &value.output);
    eval_canonical(builder, &value.quotient);
    builder.assert_eq(
        value.left.value * value.right.value,
        value.quotient.value * Value::from_u16(Q) + value.output.value,
    );
}

impl Mul<Value> {
    pub fn witness(left: u16, right: u16) -> Option<Self> {
        let (left_value, right_value) = (Canonical::witness(left)?, Canonical::witness(right)?);
        let product = u32::from(left) * u32::from(right);
        Some(Self {
            left: left_value,
            right: right_value,
            output: Canonical::witness((product % u32::from(Q)) as u16)?,
            quotient: Canonical::witness((product / u32::from(Q)) as u16)?,
        })
    }
}
