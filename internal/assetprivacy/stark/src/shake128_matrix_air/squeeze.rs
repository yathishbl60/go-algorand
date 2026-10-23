// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OUTPUT, RATE_BYTES, Shake128MatrixAir};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, NUM_KECCAK_COLS};

impl Shake128MatrixAir {
    pub(super) fn eval_squeeze<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_continuation(builder);
        for block in 0..self.blocks {
            self.eval_output(builder, block);
        }
    }

    fn eval_continuation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row: &KeccakCols<AB::Var> = main.current_slice()[..NUM_KECCAK_COLS].borrow();
        let next: &KeccakCols<AB::Var> = main.next_slice()[..NUM_KECCAK_COLS].borrow();
        let gate: AB::Expr = builder.preprocessed().current_slice()[..self.blocks - 1]
            .iter()
            .map(|&v| v.into())
            .sum();
        for limb in 0..100 {
            let output = row.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            builder
                .when_transition()
                .when(gate.clone())
                .assert_eq(next.preimage[limb / 20][limb / 4 % 5][limb % 4], output);
        }
    }

    fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let columns: &KeccakCols<AB::Var> = row[..NUM_KECCAK_COLS].borrow();
        let gate = builder.preprocessed().current_slice()[block];
        for limb in 0..RATE_BYTES / 2 {
            let start = OUTPUT + block * RATE_BYTES * 8 + limb * 16;
            let expected = (0..16)
                .rev()
                .fold(AB::Expr::ZERO, |v, bit| v.double() + row[start + bit]);
            let actual = columns.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            builder.when(gate).assert_eq(actual, expected);
        }
    }
}
