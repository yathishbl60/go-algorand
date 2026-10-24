// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    G_END, G_FIRST, H_END, HASH, KECCAK, MlKemStreamDerivationAir, OUTPUT, WINDOW, message,
};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::KeccakCols;

impl MlKemStreamDerivationAir {
    pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[KECCAK..WINDOW].borrow();
        let expand = builder.preprocessed().current_slice()[G_FIRST];
        for limb in 0..100 {
            let input = keccak.preimage[limb / 20][limb / 4 % 5][limb % 4];
            let key = initial_limb::<AB>(row, limb, false);
            let coins = initial_limb::<AB>(row, limb, true);
            builder.when_first_row().assert_eq(input, key);
            builder.when(expand).assert_eq(input, coins);
        }
    }
    pub(super) fn eval_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[KECCAK..WINDOW].borrow();
        let preprocessing = builder.preprocessed().clone();
        let pre = preprocessing.current_slice();
        for limb in 0..32 {
            let result = keccak.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            let coins = message::bits::<AB>(&row[OUTPUT + limb * 16..OUTPUT + (limb + 1) * 16]);
            builder.when(pre[G_END]).assert_eq(result, coins);
            if limb < 16 {
                let hash = message::bits::<AB>(&row[HASH + limb * 16..HASH + (limb + 1) * 16]);
                builder.when(pre[H_END]).assert_eq(result, hash);
            }
        }
    }
}

fn initial_limb<AB: AirBuilder<F = Value>>(row: &[AB::Var], limb: usize, expand: bool) -> AB::Expr {
    if limb >= if expand { 36 } else { 68 } {
        return AB::Expr::ZERO;
    }
    (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
        let input = if expand {
            message::expand_bit::<AB>(row, limb * 16 + bit)
        } else {
            message::key_bit::<AB>(row, 0, limb * 16 + bit)
        };
        value.double() + input
    })
}
