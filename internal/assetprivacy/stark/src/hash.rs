// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_symmetric::CryptographicHasher;
use sha3::{Digest, Sha3_512};

/// Separate leaf, internal-node, and transcript hash domains.
#[derive(Clone, Copy, Debug)]
pub struct DomainHash<const TAG: u8>;

impl<const TAG: u8> CryptographicHasher<u8, [u8; 64]> for DomainHash<TAG> {
    fn hash_iter<I: IntoIterator<Item = u8>>(&self, input: I) -> [u8; 64] {
        let mut hash = Sha3_512::new();
        hash.update(b"Algorand/AssetPrivacy/Candidate1/STARK/");
        hash.update([TAG]);
        for byte in input {
            hash.update([byte]);
        }
        hash.finalize().into()
    }
}

impl<const TAG: u8> CryptographicHasher<u64, [u64; 8]> for DomainHash<TAG> {
    fn hash_iter<I: IntoIterator<Item = u64>>(&self, input: I) -> [u64; 8] {
        let bytes = input.into_iter().flat_map(u64::to_le_bytes);
        let digest: [u8; 64] = self.hash_iter(bytes);
        core::array::from_fn(|i| {
            let mut word = [0; 8];
            word.copy_from_slice(&digest[i * 8..(i + 1) * 8]);
            u64::from_le_bytes(word)
        })
    }
}
