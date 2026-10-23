// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Canonical, Q, eval_canonical};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub struct Sub<T> {
    pub left: Canonical<T>,
    pub right: Canonical<T>,
    pub output: Canonical<T>,
    pub borrow: T,
}

pub fn eval_sub<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Sub<AB::Var>) {
    eval_canonical(builder, &value.left);
    eval_canonical(builder, &value.right);
    eval_canonical(builder, &value.output);
    builder.assert_bool(value.borrow);
    builder.assert_eq(
        value.left.value + value.borrow * Value::from_u16(Q),
        value.right.value + value.output.value,
    );
}

impl Sub<Value> {
    pub fn witness(left: u16, right: u16) -> Option<Self> {
        let (left_value, right_value) = (Canonical::witness(left)?, Canonical::witness(right)?);
        let borrow = left < right;
        let output = u32::from(left) + u32::from(Q) * u32::from(borrow) - u32::from(right);
        Some(Self {
            left: left_value,
            right: right_value,
            output: Canonical::witness(output as u16)?,
            borrow: Value::from_bool(borrow),
        })
    }
}
