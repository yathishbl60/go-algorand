// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Canonical, Q, eval_canonical};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// Only the ML-KEM-768 ciphertext compression widths D=4 and D=10 are supported.
pub struct Compression<T, const D: usize> {
    pub input: Canonical<T>,
    pub compressed: [T; D],
    pub rollover: T,
    pub remainder: Canonical<T>,
}

pub fn eval_compression<AB: AirBuilder<F = Value>, const D: usize>(
    builder: &mut AB,
    value: &Compression<AB::Var, D>,
) {
    if !matches!(D, 4 | 10) {
        builder.assert_zero(Value::ONE);
        return;
    }
    eval_canonical(builder, &value.input);
    eval_canonical(builder, &value.remainder);
    builder.assert_bools(value.compressed);
    builder.assert_bool(value.rollover);
    let compressed = value
        .compressed
        .into_iter()
        .rev()
        .fold(AB::Expr::ZERO, |v, b| v.double() + b);
    let scale = Value::from_u16(1 << D);
    let rounded = compressed + value.rollover * scale;
    builder.assert_eq(
        rounded * Value::from_u16(Q) + value.remainder.value,
        value.input.value * scale + Value::from_u16(1664),
    );
}

impl<const D: usize> Compression<Value, D> {
    pub fn witness(input: u16) -> Option<Self> {
        if !matches!(D, 4 | 10) {
            return None;
        }
        let canonical = Canonical::witness(input)?;
        let scale = 1u32 << D;
        let numerator = scale * u32::from(input) + 1664;
        let rounded = numerator / u32::from(Q);
        let compressed = rounded % scale;
        Some(Self {
            input: canonical,
            compressed: core::array::from_fn(|b| Value::from_bool(compressed & (1 << b) != 0)),
            rollover: Value::from_bool(rounded >= scale),
            remainder: Canonical::witness((numerator % u32::from(Q)) as u16)?,
        })
    }
}
