// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Gcm,
    witness_blocks::{self, Blocks},
    witness_bytes::bits,
    witness_hash,
};
use crate::{Value, aes256_air::ExpandedKey};

type Key = ExpandedKey<Value>;

struct Inputs {
    nonce: [u8; 12],
    associated: [u8; 64],
    plaintext: [u8; 512],
}

impl Gcm<Value> {
    /// Constructs private columns only; this never evaluates or proves the relation.
    pub fn witness(
        key: [u8; 32],
        nonce: [u8; 12],
        associated: [u8; 64],
        plaintext: [u8; 512],
    ) -> Option<Self> {
        let key = Key::witness(key);
        let blocks = witness_blocks::cipher_blocks(&key, nonce)?;
        finish(
            key,
            blocks,
            Inputs {
                nonce,
                associated,
                plaintext,
            },
        )
    }
}

fn finish(key: Key, blocks: Blocks, input: Inputs) -> Option<Gcm<Value>> {
    let ciphertext = witness_blocks::ciphertext(&blocks, input.plaintext);
    let ghash = witness_hash::hash(&blocks, input.associated, ciphertext)?;
    let tag = witness_hash::tag(&blocks, &ghash);
    Some(Gcm {
        key,
        blocks,
        ghash,
        nonce: bits(input.nonce),
        associated: bits(input.associated),
        plaintext: bits(input.plaintext),
        ciphertext: bits(ciphertext),
        tag: bits(tag),
    })
}
