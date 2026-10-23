// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Block, Gf128, block::eval_block, eval_multiply};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

/// N is fixed by the enclosing verifier, not selected by a proof.
pub struct Ghash<T, const N: usize> {
    pub hash_subkey: Block<T>,
    pub blocks: [Block<T>; N],
    pub products: Box<[Gf128<T>; N]>,
    pub output: Block<T>,
}

pub fn eval_ghash<AB: AirBuilder<F = Value>, const N: usize>(
    builder: &mut AB,
    value: &Ghash<AB::Var, N>,
) {
    builder.assert_one(Value::from_bool(N > 0));
    eval_block(builder, value.hash_subkey);
    eval_block(builder, value.output);
    for step in 0..N {
        eval_multiply(builder, &value.products[step]);
        eval_link(builder, value, step);
    }
    if N > 0 {
        for bit in 0..128 {
            builder.assert_eq(value.output[bit], value.products[N - 1].sums[127][bit]);
        }
    }
}

fn eval_link<AB: AirBuilder<F = Value>, const N: usize>(
    builder: &mut AB,
    value: &Ghash<AB::Var, N>,
    step: usize,
) {
    eval_block(builder, value.blocks[step]);
    for bit in 0..128 {
        let previous = if step == 0 {
            AB::Expr::ZERO
        } else {
            value.products[step - 1].sums[127][bit].into()
        };
        builder.assert_eq(value.products[step].right[bit], value.hash_subkey[bit]);
        builder.assert_eq(
            value.products[step].left[bit],
            previous.xor(&value.blocks[step][bit].into()),
        );
    }
}
