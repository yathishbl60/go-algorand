// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{AesGcmAir, INPUT, OUT, STATE, schedule::rounds, view};
use crate::{Value, aes_air::eval_sbox, aes256_air::eval_mix};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl AesGcmAir {
    pub(super) fn eval_cipher<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_initial(builder);
        self.eval_substitutions(builder);
        let main = builder.main();
        let row = main.current_slice();
        let shifted = view::shifted(row);
        for column in 0..4 {
            let input = core::array::from_fn(|r| shifted[4 * column + r]);
            let active = rounds(builder, 13);
            eval_mix(&mut builder.when(active), input, &view::mix(row, column));
        }
        for round in 0..14 {
            self.eval_round(builder, round);
        }
        self.eval_transition(builder);
    }

    fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let start = builder.preprocessed().current_slice()[13];
        for byte in 0..16 {
            for bit in 0..8 {
                let input = AB::Expr::from(row[INPUT + 8 * byte + bit]);
                let key = view::key(row, byte / 4, byte % 4)[bit].into();
                builder
                    .when(start)
                    .assert_eq(row[STATE + 8 * byte + bit], input.xor(&key));
            }
        }
    }

    fn eval_substitutions<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = rounds(builder, 14);
        for byte in 0..16 {
            let sbox = view::sbox(row, byte);
            eval_sbox(builder, &sbox);
            for bit in 0..8 {
                builder
                    .when(active.clone())
                    .assert_eq(sbox.inversion.left[bit], row[STATE + 8 * byte + bit]);
            }
        }
    }

    fn eval_round<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, round: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[13 + round];
        let shifted = view::shifted(row);
        for byte in 0..16 {
            let transformed = if round == 13 {
                shifted[byte]
            } else {
                view::mix(row, byte / 4).output[byte % 4]
            };
            let key = view::key(row, (round + 1) * 4 + byte / 4, byte % 4);
            for bit in 0..8 {
                let result = AB::Expr::from(transformed[bit]).xor(&key[bit].into());
                builder
                    .when(flag)
                    .assert_eq(row[OUT + 8 * byte + bit], result);
            }
        }
    }

    fn eval_transition<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let active = rounds(builder, 13);
        for bit in 0..128 {
            builder.when_transition().when(active.clone()).assert_eq(
                main.next_slice()[STATE + bit],
                main.current_slice()[OUT + bit],
            );
        }
    }
}
