// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{BOXES, H, INPUT, KEY, MASK, MIX, NONCE, OUT, PT, STATE, write};
use crate::{
    Value,
    aes_air::{Byte, SBox},
    aes256_air::{CipherBlock, ExpandedKey},
};

pub(super) struct Fixed {
    pub key: ExpandedKey<Value>,
    pub nonce: [u8; 12],
    pub plaintext: [u8; 512],
    pub hash: [Byte<Value>; 16],
    pub mask: [Byte<Value>; 16],
}

impl Fixed {
    pub fn new(key: [u8; 32], nonce: [u8; 12], plaintext: [u8; 512]) -> Self {
        let key = ExpandedKey::witness(key);
        let hash = CipherBlock::witness(&key, input(nonce, 0)).state[14];
        let mask = CipherBlock::witness(&key, input(nonce, 1)).state[14];
        Self {
            key,
            nonce,
            plaintext,
            hash,
            mask,
        }
    }

    pub fn write_fixed(&self, row: &mut [Value]) {
        for (word, values) in self.key.words.iter().enumerate() {
            write::bytes(&mut row[KEY + word * 32..KEY + (word + 1) * 32], values);
        }
        write::raw(&mut row[PT..PT + 4096], &self.plaintext);
        write::raw(&mut row[NONCE..NONCE + 96], &self.nonce);
        write::bytes(&mut row[H..H + 128], &self.hash);
        write::bytes(&mut row[MASK..MASK + 128], &self.mask);
        let zero = SBox::witness(0);
        for target in row[BOXES..MIX].as_chunks_mut::<241>().0 {
            write::sbox(target, &zero);
        }
    }

    pub fn write_key(&self, row: &mut [Value], stage: usize) {
        for (index, value) in self.key.substitutions[stage].iter().enumerate() {
            write::sbox(
                &mut row[BOXES + index * 241..BOXES + (index + 1) * 241],
                value,
            );
        }
    }
}

pub(super) fn input(nonce: [u8; 12], block: usize) -> [u8; 16] {
    let mut value = [0; 16];
    if block != 0 {
        value[..12].copy_from_slice(&nonce);
        value[12..].copy_from_slice(&(block as u32).to_be_bytes());
    }
    value
}

pub(super) fn write_round(row: &mut [Value], block: &CipherBlock<Value>, round: usize) {
    write::bytes(&mut row[INPUT..INPUT + 128], &block.plaintext);
    write::bytes(&mut row[STATE..STATE + 128], &block.state[round]);
    write::bytes(&mut row[OUT..OUT + 128], &block.state[round + 1]);
    for (index, value) in block.substitutions[round].iter().enumerate() {
        write::sbox(
            &mut row[BOXES + index * 241..BOXES + (index + 1) * 241],
            value,
        );
    }
    if round < 13 {
        for (index, value) in block.mix[round].iter().enumerate() {
            write::mix(&mut row[MIX + index * 152..MIX + (index + 1) * 152], value);
        }
    }
}
