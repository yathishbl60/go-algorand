// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Byte, GfMul, affine::eval_affine, eval_multiply};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// 241 columns. Input=inversion.left; inverse=inversion.right; output is ranged.
pub struct SBox<T> {
    pub inversion: GfMul<T>,
    pub is_zero: T,
    pub affine: [Byte<T>; 4],
    pub output: Byte<T>,
}

/// Constrains the AES S-box including the required 0 -> 0x63 case.
pub fn eval_sbox<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &SBox<AB::Var>) {
    eval_multiply(builder, &value.inversion);
    builder.assert_bool(value.is_zero);
    for bit in 0..8 {
        builder.assert_zero(value.is_zero * value.inversion.left[bit]);
        builder.assert_zero(value.is_zero * value.inversion.right[bit]);
        let expected = if bit == 0 {
            AB::Expr::ONE - value.is_zero
        } else {
            AB::Expr::ZERO
        };
        builder.assert_eq(value.inversion.sums[7][bit], expected);
    }
    eval_affine(builder, value);
}
