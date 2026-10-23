// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ExpandedKey,
    byte::{bits, integer},
};
use crate::{Value, aes_air::SBox};
use p3_field::PrimeCharacteristicRing;

impl ExpandedKey<Value> {
    pub fn witness(key: [u8; 32]) -> Self {
        let mut words = [[[Value::ZERO; 8]; 4]; 60];
        for i in 0..32 {
            words[i / 4][i % 4] = bits(key[i]);
        }
        let substitutions = core::array::from_fn(|stage| {
            let word = 8 + 4 * stage;
            let substitution = substitutions(&words, word);
            let mut temporary = core::array::from_fn(|i| integer(substitution[i].output));
            if word.is_multiple_of(8) {
                temporary[0] ^= 1 << (word / 8 - 1);
            }
            expand_group(&mut words, word, temporary);
            substitution
        });
        Self {
            words,
            substitutions,
        }
    }
}

fn substitutions(words: &[[[Value; 8]; 4]; 60], word: usize) -> [SBox<Value>; 4] {
    core::array::from_fn(|byte| {
        let source = if word.is_multiple_of(8) {
            (byte + 1) % 4
        } else {
            byte
        };
        SBox::witness(integer(words[word - 1][source]))
    })
}

fn expand_group(words: &mut [[[Value; 8]; 4]; 60], first: usize, mut temporary: [u8; 4]) {
    for word in first..first + 4 {
        for byte in 0..4 {
            words[word][byte] = bits(integer(words[word - 8][byte]) ^ temporary[byte]);
        }
        temporary = words[word].map(integer);
    }
}
