// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::key_binding::{eval_substitution, eval_word};
use crate::{
    Value,
    aes_air::{Byte, SBox, eval_byte},
};
use p3_air::AirBuilder;

/// 60 words (240 bytes), with the original 32-byte key in words[0..8].
/// The 52 S-boxes implement RotWord/SubWord and the extra AES-256 SubWord cases.
pub struct ExpandedKey<T> {
    pub words: [[Byte<T>; 4]; 60],
    pub substitutions: [[SBox<T>; 4]; 13],
}

pub fn eval_key<AB: AirBuilder<F = Value>>(builder: &mut AB, key: &ExpandedKey<AB::Var>) {
    for word in key.words {
        for byte in word {
            eval_byte(builder, byte);
        }
    }
    for word in 8usize..60 {
        if word.is_multiple_of(4) {
            eval_substitution(builder, key, word);
        }
        eval_word(builder, key, word);
    }
}
