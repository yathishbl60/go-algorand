// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CIPHER_BLOCKS, PAYLOAD_BYTES, witness_bytes::block_output};
use crate::{
    Value,
    aes256_air::{CipherBlock, ExpandedKey},
};

pub(super) type Blocks = Box<[CipherBlock<Value>; CIPHER_BLOCKS]>;

pub(super) fn cipher_blocks(key: &ExpandedKey<Value>, nonce: [u8; 12]) -> Option<Blocks> {
    let blocks: Vec<_> = (0..CIPHER_BLOCKS)
        .map(|index| CipherBlock::witness(key, counter(nonce, index)))
        .collect();
    blocks.into_boxed_slice().try_into().ok()
}

fn counter(nonce: [u8; 12], index: usize) -> [u8; 16] {
    if index == 0 {
        return [0; 16];
    }
    let mut block = [0; 16];
    block[..12].copy_from_slice(&nonce);
    block[12..].copy_from_slice(&(index as u32).to_be_bytes());
    block
}

pub(super) fn ciphertext(blocks: &Blocks, plaintext: [u8; PAYLOAD_BYTES]) -> [u8; PAYLOAD_BYTES] {
    let streams: [[u8; 16]; 32] = core::array::from_fn(|block| block_output(&blocks[block + 2]));
    core::array::from_fn(|byte| plaintext[byte] ^ streams[byte / 16][byte % 16])
}
