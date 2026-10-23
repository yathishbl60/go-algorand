// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, NUM_KECCAK_COLS};

use super::{RATE_BYTES, Sha3Air, output_bits::final_bit};
use crate::Value;

impl Sha3Air {
    pub(super) fn eval_absorption<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for block in 1..self.blocks() {
            self.eval_absorbed_block(builder, block);
        }
    }

    fn eval_absorbed_block<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let local: &KeccakCols<AB::Var> = main.current_slice()[..NUM_KECCAK_COLS].borrow();
        let next: &KeccakCols<AB::Var> = main.next_slice()[..NUM_KECCAK_COLS].borrow();
        let active = main.current_slice()[self.block_offset() + block - 1];
        let gate = builder.is_transition() * local.step_flags[23] * active;
        for limb in 0..100 {
            let absorbed = self.absorbed_limb::<AB>(main.current_slice(), block, limb);
            builder
                .when(gate.clone())
                .assert_eq(next.preimage[limb / 20][limb / 4 % 5][limb % 4], absorbed);
        }
    }

    fn absorbed_limb<AB: AirBuilder<F = Value>>(
        &self,
        row: &[AB::Var],
        block: usize,
        limb: usize,
    ) -> AB::Expr {
        let columns: &KeccakCols<AB::Var> = row[..NUM_KECCAK_COLS].borrow();
        if limb >= RATE_BYTES / 2 {
            return columns
                .a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4)
                .into();
        }
        (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
            let index = limb * 16 + bit;
            let message = self.message_bit::<AB>(row, block * RATE_BYTES * 8 + index);
            value.double() + final_bit::<AB>(columns, index).xor(&message)
        })
    }
}
