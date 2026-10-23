// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact candidate 64-byte-AAD/512-byte-payload AES-256-GCM constraints.
//! KEM, KDF, note/policy bindings and practical trace scheduling remain separate.

mod cipher;
mod hash;
mod range;
mod witness;
mod witness_blocks;
mod witness_bytes;
mod witness_hash;

use crate::{
    aes_air::Byte,
    aes256_air::{CipherBlock, ExpandedKey},
    ghash_air::Ghash,
};

pub const AAD_BYTES: usize = 64;
pub const PAYLOAD_BYTES: usize = 512;
pub const CIPHER_BLOCKS: usize = 34;
pub const HASH_BLOCKS: usize = 37;

/// Typed columns only; a parent AIR allocates, binds and evaluates this component.
pub struct Gcm<T> {
    pub key: ExpandedKey<T>,
    pub blocks: Box<[CipherBlock<T>; CIPHER_BLOCKS]>,
    pub ghash: Ghash<T, HASH_BLOCKS>,
    pub nonce: [Byte<T>; 12],
    pub associated: [Byte<T>; AAD_BYTES],
    pub plaintext: [Byte<T>; PAYLOAD_BYTES],
    pub ciphertext: [Byte<T>; PAYLOAD_BYTES],
    pub tag: [Byte<T>; 16],
}

pub use cipher::eval_gcm;
