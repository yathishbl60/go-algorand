// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DIGEST, EnvelopeAadAir, KECCAK, OUTPUT, WINDOW, message};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::KeccakCols;

impl EnvelopeAadAir {
    pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[KECCAK..WINDOW].borrow();
        for limb in 0..100 {
            let expected = self.initial_limb::<AB>(row, limb);
            builder
                .when_first_row()
                .assert_eq(keccak.preimage[limb / 20][limb / 4 % 5][limb % 4], expected);
        }
    }
    fn initial_limb<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var], limb: usize) -> AB::Expr {
        if limb >= 36 {
            return AB::Expr::ZERO;
        }
        (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
            value.double() + self.message_bit::<AB>(row, 0, limb * 16 + bit)
        })
    }
    pub(super) fn eval_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[KECCAK..WINDOW].borrow();
        let flag = builder.preprocessed().current_slice()[DIGEST];
        for limb in 0..32 {
            let actual = keccak.a_prime_prime_prime(limb / 20, limb / 4 % 5, limb % 4);
            let digest = message::bits::<AB>(&row[OUTPUT + 16 * limb..OUTPUT + 16 * (limb + 1)]);
            builder.when(flag).assert_eq(actual, digest);
        }
    }
}
