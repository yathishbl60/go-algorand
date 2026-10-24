// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Encapsulation, key::ParsedKey, witness};
use sha3::{Digest, Sha3_256, Sha3_512};

/// Caller supplies a fresh secret uniform m; no randomness is generated here.
pub fn encapsulate(public_key: &[u8], message: [u8; 32]) -> Option<Encapsulation> {
    let key = ParsedKey::parse(public_key)?;
    let key_hash = Sha3_256::digest(public_key).into();
    let (shared_key, coins) = expand(message, key_hash);
    let encryption = witness::encrypt(key, message, coins)?;
    Some(Encapsulation {
        key_hash,
        shared_key,
        coins,
        encryption,
    })
}

fn expand(message: [u8; 32], key_hash: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    let mut hash = Sha3_512::new();
    hash.update(message);
    hash.update(key_hash);
    let digest: [u8; 64] = hash.finalize().into();
    let shared_key = core::array::from_fn(|index| digest[index]);
    let coins = core::array::from_fn(|index| digest[32 + index]);
    (shared_key, coins)
}
