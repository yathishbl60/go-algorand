// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{AesGcmAir, view};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl AesGcmAir {
    pub(super) fn eval_key<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for word in 8..60 {
            if word % 4 != 0 { self.eval_regular(builder, word); }
        }
        for stage in 0..13 { self.eval_subword(builder, stage); }
    }

    fn eval_regular<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, word: usize) {
        let main = builder.main();
        let row = main.current_slice();
        for byte in 0..4 {
            for bit in 0..8 {
                let prior = AB::Expr::from(view::key(row, word - 8, byte)[bit]);
                let temp = view::key(row, word - 1, byte)[bit].into();
                builder.assert_eq(view::key(row, word, byte)[bit], prior.xor(&temp));
            }
        }
    }

    fn eval_subword<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, stage: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[stage];
        let word = 8 + 4 * stage;
        for byte in 0..4 {
            let box_value = view::sbox(row, byte);
            let source = if word % 8 == 0 { (byte + 1) % 4 } else { byte };
            for bit in 0..8 {
                builder.when(flag).assert_eq(box_value.inversion.left[bit], view::key(row, word - 1, source)[bit]);
                let rcon = word % 8 == 0 && byte == 0 && bit == word / 8 - 1;
                let temp = AB::Expr::from(box_value.output[bit]).xor(&AB::Expr::from_bool(rcon));
                let prior = AB::Expr::from(view::key(row, word - 8, byte)[bit]);
                builder.when(flag).assert_eq(view::key(row, word, byte)[bit], prior.xor(&temp));
            }
        }
    }
}
