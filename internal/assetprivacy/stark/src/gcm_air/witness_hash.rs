// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD_BYTES, HASH_BLOCKS, PAYLOAD_BYTES};
use super::{
    witness_blocks::Blocks,
    witness_bytes::{block_output, hash_output},
};
use crate::{Value, ghash_air::Ghash};

type Hash = Ghash<Value, HASH_BLOCKS>;

pub(super) fn hash(
    blocks: &Blocks,
    aad: [u8; AAD_BYTES],
    ciphertext: [u8; PAYLOAD_BYTES],
) -> Option<Hash> {
    let message = core::array::from_fn(|block| hash_block(&aad, &ciphertext, block));
    Ghash::witness(block_output(&blocks[0]), message)
}

fn hash_block(aad: &[u8; AAD_BYTES], ciphertext: &[u8; PAYLOAD_BYTES], block: usize) -> [u8; 16] {
    if block < 4 {
        return aad[block * 16..(block + 1) * 16]
            .try_into()
            .expect("AAD block");
    }
    if block < 36 {
        let start = (block - 4) * 16;
        return ciphertext[start..start + 16]
            .try_into()
            .expect("ciphertext block");
    }
    let mut lengths = [0; 16];
    lengths[..8].copy_from_slice(&(AAD_BYTES as u64 * 8).to_be_bytes());
    lengths[8..].copy_from_slice(&(PAYLOAD_BYTES as u64 * 8).to_be_bytes());
    lengths
}

pub(super) fn tag(blocks: &Blocks, hash: &Hash) -> [u8; 16] {
    let hash = hash_output(hash.output);
    let mask = block_output(&blocks[1]);
    core::array::from_fn(|byte| hash[byte] ^ mask[byte])
}
