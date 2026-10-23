// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SBox, eval_byte};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub(super) fn eval_affine<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &SBox<AB::Var>) {
    for step in 0..4 {
        eval_step(builder, value, step);
    }
    eval_byte(builder, value.output);
    for bit in 0..8 {
        let constant = AB::Expr::from_bool(0x63 & (1 << bit) != 0);
        builder.assert_eq(
            value.output[bit],
            AB::Expr::from(value.affine[3][bit]).xor(&constant),
        );
    }
}

fn eval_step<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &SBox<AB::Var>, step: usize) {
    let inverse = value.inversion.right;
    eval_byte(builder, value.affine[step]);
    for (bit, inverse_bit) in inverse.into_iter().enumerate() {
        let prior: AB::Expr = if step == 0 {
            inverse_bit.into()
        } else {
            value.affine[step - 1][bit].into()
        };
        builder.assert_eq(
            value.affine[step][bit],
            prior.xor(&inverse[(bit + step + 4) % 8].into()),
        );
    }
}
