// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, NUM_KECCAK_COLS};

use super::Sha3Air;
use crate::Value;

impl Sha3Air {
    pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row: &KeccakCols<AB::Var> = main.current_slice()[..NUM_KECCAK_COLS].borrow();
        for limb in 0..100 {
            let expected = self.initial_limb::<AB>(main.current_slice(), limb);
            builder
                .when_first_row()
                .assert_eq(row.preimage[limb / 20][limb / 4 % 5][limb % 4], expected);
        }
    }

    fn initial_limb<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var], limb: usize) -> AB::Expr {
        if limb >= self.rate_bytes() / 2 {
            return AB::Expr::ZERO;
        }
        (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
            value.double() + self.message_bit::<AB>(row, limb * 16 + bit)
        })
    }

    pub(super) fn eval_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row: &KeccakCols<AB::Var> = main.current_slice()[..NUM_KECCAK_COLS].borrow();
        let last_block = main.current_slice()[self.block_offset() + self.blocks() - 1];
        let gate = last_block * row.step_flags[23];
        for limb in 0..self.digest_bytes / 2 {
            let expected = builder.public_values()[limb];
            let value = row.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            builder.when(gate.clone()).assert_eq(value, expected);
        }
    }
}
