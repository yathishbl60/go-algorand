// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ExpandedKey;
use crate::{Value, aes_air::eval_sbox};
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub(super) fn eval_substitution<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    key: &ExpandedKey<AB::Var>,
    word: usize,
) {
    for byte in 0..4 {
        let sbox = &key.substitutions[(word - 8) / 4][byte];
        eval_sbox(builder, sbox);
        let source = if word.is_multiple_of(8) {
            (byte + 1) % 4
        } else {
            byte
        };
        for bit in 0..8 {
            builder.assert_eq(sbox.inversion.left[bit], key.words[word - 1][source][bit]);
        }
    }
}

pub(super) fn eval_word<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    key: &ExpandedKey<AB::Var>,
    word: usize,
) {
    for byte in 0..4 {
        for bit in 0..8 {
            let prior = AB::Expr::from(key.words[word - 8][byte][bit]);
            let temporary = temp::<AB>(key, word, byte, bit);
            builder.assert_eq(key.words[word][byte][bit], prior.xor(&temporary));
        }
    }
}

fn temp<AB: AirBuilder<F = Value>>(
    key: &ExpandedKey<AB::Var>,
    word: usize,
    byte: usize,
    bit: usize,
) -> AB::Expr {
    if !word.is_multiple_of(4) {
        return key.words[word - 1][byte][bit].into();
    }
    let substituted: AB::Expr = key.substitutions[(word - 8) / 4][byte].output[bit].into();
    let rcon = word.is_multiple_of(8) && byte == 0 && bit == word / 8 - 1;
    substituted.xor(&AB::Expr::from_bool(rcon))
}
