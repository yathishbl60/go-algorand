// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ExpandedKey, MixColumn,
    add_key::{eval_initial, eval_round_key},
    eval_key, eval_mix,
};
use crate::{
    Value,
    aes_air::{Byte, SBox, eval_byte, eval_sbox},
};
use p3_air::AirBuilder;

pub type States<T> = [[Byte<T>; 16]; 15];
pub type MixRounds<T> = [[MixColumn<T>; 4]; 13];

/// A complete AES-256 block component. Ciphertext is state[14].
pub struct Aes256<T> {
    pub key: ExpandedKey<T>,
    pub plaintext: [Byte<T>; 16],
    pub state: States<T>,
    pub substitutions: [[SBox<T>; 16]; 14],
    pub mix: MixRounds<T>,
}

pub fn eval_cipher<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Aes256<AB::Var>) {
    eval_key(builder, &value.key);
    for byte in value.plaintext {
        eval_byte(builder, byte);
    }
    for state in value.state {
        for byte in state {
            eval_byte(builder, byte);
        }
    }
    eval_initial(builder, value);
    for round in 0..14 {
        eval_round(builder, value, round);
    }
}

fn eval_round<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Aes256<AB::Var>, round: usize) {
    for byte in 0..16 {
        let sbox = &value.substitutions[round][byte];
        eval_sbox(builder, sbox);
        for bit in 0..8 {
            builder.assert_eq(sbox.inversion.left[bit], value.state[round][byte][bit]);
        }
    }
    let shifted = shifted(&value.substitutions[round]);
    if round < 13 {
        for column in 0..4 {
            let input = core::array::from_fn(|r| shifted[4 * column + r]);
            eval_mix(builder, input, &value.mix[round][column]);
        }
    }
    eval_round_key(builder, value, round, shifted);
}

pub(super) fn shifted<T: Copy>(substitutions: &[SBox<T>; 16]) -> [Byte<T>; 16] {
    core::array::from_fn(|i| substitutions[i % 4 + 4 * ((i / 4 + i % 4) % 4)].output)
}
