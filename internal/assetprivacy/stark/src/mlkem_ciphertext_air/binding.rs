// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemCiphertextAir, SCRATCH, WORD_BITS};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemCiphertextAir {
    pub(super) fn eval_input<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (current, next) in row[..COEFFICIENTS]
            .iter()
            .zip(&main.next_slice()[..COEFFICIENTS])
        {
            builder.when_transition().assert_eq(*current, *next);
        }
        let prep = builder.preprocessed();
        let selected: AB::Expr = (0..COEFFICIENTS)
            .map(|i| prep.current_slice()[i] * row[i])
            .sum();
        builder.assert_eq(row[SCRATCH], selected);
    }

    pub(super) fn eval_words<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[WORD_BITS..WORD_BITS + 32] {
            builder.assert_bool(bit);
        }
        for word in 0..2 {
            let decoded = (0..16).rev().fold(AB::Expr::ZERO, |v, bit| {
                v.double() + row[WORD_BITS + word * 16 + bit]
            });
            let expected = super::packing::selected_word::<AB>(builder, word);
            builder.assert_eq(decoded, expected);
        }
    }
}
