// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Canonical, Q, eval_canonical};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub struct Add<T> {
    pub left: Canonical<T>,
    pub right: Canonical<T>,
    pub output: Canonical<T>,
    pub carry: T,
}

pub fn eval_add<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Add<AB::Var>) {
    eval_canonical(builder, &value.left);
    eval_canonical(builder, &value.right);
    eval_canonical(builder, &value.output);
    builder.assert_bool(value.carry);
    builder.assert_eq(
        value.left.value + value.right.value,
        value.output.value + value.carry * Value::from_u16(Q),
    );
}

impl Add<Value> {
    pub fn witness(left: u16, right: u16) -> Option<Self> {
        let (left_value, right_value) = (Canonical::witness(left)?, Canonical::witness(right)?);
        let sum = u32::from(left) + u32::from(right);
        Some(Self {
            left: left_value,
            right: right_value,
            output: Canonical::witness((sum % u32::from(Q)) as u16)?,
            carry: Value::from_bool(sum >= u32::from(Q)),
        })
    }
}
