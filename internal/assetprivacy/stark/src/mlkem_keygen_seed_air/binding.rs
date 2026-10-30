// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DIGEST, HASH_WIDTH, MlKemKeygenSeedAir, RHO, WIDTH};
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl MlKemKeygenSeedAir {
    pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        self.hash.eval(&mut PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..HASH_WIDTH,
            output: DIGEST..DIGEST + 32,
        });
    }
    pub(super) fn eval_dimension<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for bit in 0..8 {
            builder.assert_eq(
                main.current_slice()[NUM_KECCAK_COLS + 256 + bit],
                Value::from_bool(3 & (1 << bit) != 0),
            );
        }
    }
    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (offset, &current) in row[DIGEST..WIDTH].iter().enumerate() {
            builder
                .when_transition()
                .assert_eq(current, main.next_slice()[DIGEST + offset]);
        }
        for &bit in &row[RHO..WIDTH] {
            builder.assert_bool(bit);
        }
        for limb in 0..32 {
            let start = RHO + 16 * limb;
            let value = (0..16)
                .rev()
                .fold(AB::Expr::ZERO, |v, bit| v.double() + row[start + bit]);
            builder.assert_eq(row[DIGEST + limb], value);
        }
    }
}
