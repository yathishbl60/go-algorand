// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, BITS, DATA, GhashChainAir};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

const LENGTHS: [u8; 16] = [0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0];

impl GhashChainAir {
    pub(super) fn eval_data<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for block in 0..37 {
            if (4..36).contains(&block) {
                self.eval_ciphertext(builder, block);
            } else {
                self.eval_private_or_length(builder, block);
            }
        }
    }

    fn eval_private_or_length<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let gate = builder.preprocessed().current_slice()[BITS + block];
        for bit in 0..BITS {
            let expected = if block < 4 {
                row[AAD + block * BITS + bit].into()
            } else {
                AB::Expr::from_bool(LENGTHS[bit / 8] & (1 << (7 - bit % 8)) != 0)
            };
            builder.when(gate).assert_eq(row[DATA + bit], expected);
        }
    }

    fn eval_ciphertext<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let gate = builder.preprocessed().current_slice()[BITS + block];
        for limb in 0..8 {
            let value = (0..16).rev().fold(AB::Expr::ZERO, |v, bit| {
                let offset = limb * 16 + bit / 8 * 8 + 7 - bit % 8;
                v.double() + row[DATA + offset]
            });
            let public = builder.public_values()[(block - 4) * 8 + limb];
            builder.when(gate).assert_eq(value, public);
        }
    }
}
