// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{AesGcmAir, H, MASK, OUT, PT};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl AesGcmAir {
    pub(super) fn eval_outputs<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (block, start) in [(0, H), (1, MASK)] {
            let prep = builder.preprocessed();
            let flag = prep.current_slice()[26] * prep.current_slice()[27 + block];
            for bit in 0..128 {
                builder
                    .when(flag.clone())
                    .assert_eq(row[start + bit], row[OUT + bit]);
            }
        }
        for block in 2..34 {
            self.eval_ciphertext(builder, block);
        }
    }

    fn eval_ciphertext<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let flag = prep.current_slice()[26] * prep.current_slice()[27 + block];
        let start = (block - 2) * 128;
        for limb in 0..8 {
            let decoded = (0..16).rev().fold(AB::Expr::ZERO, |v, bit| {
                let plaintext = AB::Expr::from(row[PT + start + limb * 16 + bit]);
                v.double() + plaintext.xor(&row[OUT + limb * 16 + bit].into())
            });
            let public = builder.public_values()[12 + (block - 2) * 8 + limb];
            builder.when(flag.clone()).assert_eq(decoded, public);
        }
    }
}
