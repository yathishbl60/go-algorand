// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ExpandedKey, MixColumn,
    add_key::key_byte,
    byte::{integer, xor},
    round::{MixRounds, States, shifted},
};
use crate::{
    Value,
    aes_air::{Byte, SBox},
};

pub(super) struct Rounds {
    pub state: Vec<[Byte<Value>; 16]>,
    pub mix: Vec<[MixColumn<Value>; 4]>,
}

impl Rounds {
    pub fn new(key: &ExpandedKey<Value>, plaintext: [Byte<Value>; 16]) -> Self {
        let initial = core::array::from_fn(|i| xor(plaintext[i], key_byte(key, 0, i)));
        Self {
            state: vec![initial],
            mix: Vec::with_capacity(13),
        }
    }

    pub fn finish(self) -> (States<Value>, MixRounds<Value>) {
        let state = self
            .state
            .try_into()
            .unwrap_or_else(|_| unreachable!("15 AES states"));
        let mix = self
            .mix
            .try_into()
            .unwrap_or_else(|_| unreachable!("13 MixColumns rounds"));
        (state, mix)
    }

    pub fn step(&mut self, key: &ExpandedKey<Value>, round: usize) -> [SBox<Value>; 16] {
        let previous = self.state[round];
        let substitutions = core::array::from_fn(|i| SBox::witness(integer(previous[i])));
        let mut transformed = shifted(&substitutions);
        if round < 13 {
            let mix = core::array::from_fn(|c| {
                MixColumn::witness(core::array::from_fn(|r| transformed[4 * c + r]))
            });
            transformed = core::array::from_fn(|i| mix[i / 4].output[i % 4]);
            self.mix.push(mix);
        }
        self.state.push(core::array::from_fn(|i| {
            xor(transformed[i], key_byte(key, round + 1, i))
        }));
        substitutions
    }
}
