// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{END, KECCAK, MlKemStreamDerivationAir, WINDOW, message};
use crate::{Value, sha3_air::final_bit};
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::KeccakCols;

impl MlKemStreamDerivationAir {
    pub(super) fn eval_absorption<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for block in 1..9 {
            self.eval_block(builder, block);
        }
    }
    fn eval_block<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let next = main.next_slice();
        let current: &KeccakCols<AB::Var> = row[KECCAK..WINDOW].borrow();
        let output: &KeccakCols<AB::Var> = next[KECCAK..WINDOW].borrow();
        let active = builder.preprocessed().current_slice()[END + block - 1];
        for limb in 0..100 {
            let expected = absorbed_limb::<AB>(current, next, block, limb);
            builder
                .when_transition()
                .when(active)
                .assert_eq(output.preimage[limb / 20][limb / 4 % 5][limb % 4], expected);
        }
    }
}

fn absorbed_limb<AB: AirBuilder<F = Value>>(
    current: &KeccakCols<AB::Var>,
    next: &[AB::Var],
    block: usize,
    limb: usize,
) -> AB::Expr {
    if limb >= 68 {
        return current
            .a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4)
            .into();
    }
    (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
        let index = limb * 16 + bit;
        let input = message::key_bit::<AB>(next, block, index);
        value.double() + final_bit::<AB>(current, index).xor(&input)
    })
}
