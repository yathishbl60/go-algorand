// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BUFFER, CURSOR, FRAME, MlKemStreamMatrixAir, OUTPUT, RATE, RATE_CARRY, SEED};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::KeccakCols;

impl MlKemStreamMatrixAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in row[SEED..BUFFER].iter().chain(&row[RATE..OUTPUT]) {
            builder.assert_bool(bit);
        }
        for index in (SEED..BUFFER).chain(OUTPUT..CURSOR) {
            builder
                .when_transition()
                .assert_eq(row[index], main.next_slice()[index]);
        }
    }
    pub(super) fn eval_buffer<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[..SEED].borrow();
        let last = builder.preprocessed().current_slice()[FRAME + 2];
        for limb in 0..100 {
            let current = row[BUFFER + limb];
            let result = keccak.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            builder.when_first_row().assert_zero(current);
            builder.when_transition().assert_eq(
                main.next_slice()[BUFFER + limb],
                current + last * (result - current),
            );
        }
    }
    pub(super) fn eval_rate<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let preprocessing = builder.preprocessed().clone();
        let schedule = preprocessing.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[..SEED].borrow();
        for limb in 0..84 {
            let value = bits::<AB>(&row[RATE + 16 * limb..RATE + 16 * (limb + 1)]);
            let expected = keccak.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            builder.when(schedule[FRAME + 2]).assert_eq(value, expected);
        }
        for (bit, &value) in row.iter().enumerate().take(OUTPUT).skip(RATE) {
            builder
                .when_transition()
                .when(schedule[RATE_CARRY])
                .assert_eq(value, main.next_slice()[bit]);
        }
    }
}

pub(super) fn bits<AB: AirBuilder<F = Value>>(bits: &[AB::Var]) -> AB::Expr {
    bits.iter()
        .enumerate()
        .map(|(index, &bit)| bit * Value::from_u16(1 << index))
        .sum()
}
