// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, LOCAL_BITS, MlKemCiphertextAir, PUBLIC_VALUES, WORD_BITS, bit_position};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

pub(super) fn selected_word<AB: AirBuilder<F = Value>>(builder: &AB, word: usize) -> AB::Expr {
    let prep = builder.preprocessed();
    (0..COEFFICIENTS)
        .map(|index| {
            let limb = bit_position(index) / 16 + word;
            let public = if limb < PUBLIC_VALUES {
                builder.public_values()[limb].into()
            } else {
                AB::Expr::ZERO
            };
            prep.current_slice()[index] * public
        })
        .sum()
}

impl MlKemCiphertextAir {
    pub(super) fn eval_packing<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for bit in 0..12 {
            self.eval_bit(builder, bit);
        }
    }

    fn eval_bit<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, bit: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let prep = builder.preprocessed();
        let expected: AB::Expr = (0..COEFFICIENTS)
            .filter(|&i| bit < if i < 768 { 10 } else { 4 })
            .map(|i| prep.current_slice()[i] * row[WORD_BITS + bit_position(i) % 16 + bit])
            .sum();
        builder.assert_bool(row[LOCAL_BITS + bit]);
        builder.assert_eq(row[LOCAL_BITS + bit], expected);
    }
}
