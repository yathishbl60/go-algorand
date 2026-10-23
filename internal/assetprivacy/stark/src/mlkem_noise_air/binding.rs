// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemNoiseAir;
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemNoiseAir {
    pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let start = self.digest_start();
        self.hash.eval(&mut PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..start,
            output: start..start + 64,
        });
    }

    pub(super) fn eval_nonce<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for bit in 0..8 {
            let column = self
                .hash
                .message_bit_column(32, bit)
                .expect("fixed 33-byte message");
            builder.assert_eq(
                main.current_slice()[column],
                Value::from_bool(self.nonce & (1 << bit) != 0),
            );
        }
    }

    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for bit in 0..1024 {
            let column = self.bit_start() + bit;
            builder.assert_bool(main.current_slice()[column]);
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        for limb in 0..64 {
            let reconstructed = (0..16).rev().fold(AB::Expr::ZERO, |v, bit| {
                v.double() + main.current_slice()[self.bit_start() + 16 * limb + bit]
            });
            builder.assert_eq(
                main.current_slice()[self.digest_start() + limb],
                reconstructed,
            );
        }
    }
}
