// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CipherBlock, ExpandedKey, byte::eval_xor};
use crate::{Value, aes_air::Byte};
use p3_air::AirBuilder;

pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    key: &ExpandedKey<AB::Var>,
    value: &CipherBlock<AB::Var>,
) {
    for byte in 0..16 {
        eval_xor(
            builder,
            value.plaintext[byte],
            key_byte(key, 0, byte),
            value.state[0][byte],
        );
    }
}

pub(super) fn eval_round_key<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    key: &ExpandedKey<AB::Var>,
    value: &CipherBlock<AB::Var>,
    round: usize,
    shifted: [Byte<AB::Var>; 16],
) {
    for (byte, shifted) in shifted.into_iter().enumerate() {
        let input = if round < 13 {
            value.mix[round][byte / 4].output[byte % 4]
        } else {
            shifted
        };
        let key = key_byte(key, round + 1, byte);
        eval_xor(builder, input, key, value.state[round + 1][byte]);
    }
}

pub(super) fn key_byte<T: Copy>(key: &ExpandedKey<T>, round: usize, byte: usize) -> Byte<T> {
    key.words[4 * round + byte / 4][byte % 4]
}
