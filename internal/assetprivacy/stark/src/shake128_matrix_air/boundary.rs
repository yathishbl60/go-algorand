// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SEED, Shake128MatrixAir};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, NUM_KECCAK_COLS};

impl Shake128MatrixAir {
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
        if limb < 16 {
            return (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
                value.double() + row[SEED + limb * 16 + bit]
            });
        }
        let constant = match limb {
            16 => u16::from(self.first) | (u16::from(self.second) << 8),
            17 => 0x001f,
            83 => 0x8000,
            _ => 0,
        };
        AB::Expr::from(Value::from_u16(constant))
    }
}
