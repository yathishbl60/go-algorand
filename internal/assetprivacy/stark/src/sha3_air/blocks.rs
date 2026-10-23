// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, NUM_KECCAK_COLS};

use super::Sha3Air;
use crate::Value;

impl Sha3Air {
    pub(super) fn eval_blocks<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for block in 0..self.blocks() {
            self.eval_block(builder, block);
        }
    }

    fn eval_block<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row: &KeccakCols<AB::Var> = main.current_slice()[..NUM_KECCAK_COLS].borrow();
        let offset = self.block_offset();
        let local = main.current_slice()[offset + block];
        let previous = if block == 0 {
            AB::Expr::ZERO
        } else {
            main.current_slice()[offset + block - 1].into()
        };
        let final_step: AB::Expr = row.step_flags[23].into();
        let next = local * (AB::Expr::ONE - final_step.clone()) + previous * final_step;
        builder
            .when_first_row()
            .assert_eq(local, Value::from_bool(block == 0));
        builder.when_last_row().assert_zero(local);
        builder
            .when_transition()
            .assert_eq(next, main.next_slice()[offset + block]);
    }
}
