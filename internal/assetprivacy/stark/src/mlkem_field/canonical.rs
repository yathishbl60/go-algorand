// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Q;
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// 25 columns. Both decompositions are little-endian 12-bit unsigned integers.
pub struct Canonical<T> {
    pub value: T,
    pub bits: [T; 12],
    pub gap: [T; 12],
}

pub fn eval_canonical<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Canonical<AB::Var>) {
    builder.assert_bools(value.bits);
    builder.assert_bools(value.gap);
    let decoded = decode::<AB>(value.bits);
    let gap = decode::<AB>(value.gap);
    builder.assert_eq(value.value, decoded);
    builder.assert_eq(value.value + gap, Value::from_u16(Q - 1));
}

fn decode<AB: AirBuilder<F = Value>>(bits: [AB::Var; 12]) -> AB::Expr {
    bits.into_iter()
        .rev()
        .fold(AB::Expr::ZERO, |v, bit| v.double() + bit)
}

impl Canonical<Value> {
    pub fn witness(value: u16) -> Option<Self> {
        if value >= Q {
            return None;
        }
        Some(Self {
            value: Value::from_u16(value),
            bits: bits(value),
            gap: bits(Q - 1 - value),
        })
    }
}

fn bits(value: u16) -> [Value; 12] {
    core::array::from_fn(|bit| Value::from_bool(value & (1 << bit) != 0))
}
