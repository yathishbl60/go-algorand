// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::SpendableNoteAir;
use sha3::{Digest, Sha3_512};

pub(super) fn owner_prefix(network: [u8; 32]) -> [u8; 77] {
    let mut prefix = [0; 77];
    prefix[..45].copy_from_slice(b"Algorand/AssetPrivacy/Candidate1/SpendingKey/");
    prefix[45..].copy_from_slice(&network);
    prefix
}

pub(super) fn nullifier_prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 149] {
    crate::nullifier_v2::prefix(network, asset, policy)
}

impl SpendableNoteAir {
    pub(super) fn owner_message(&self, opening: &[u8; 512], secret: [u8; 32]) -> Option<[u8; 109]> {
        if secret == [0; 32] {
            return None;
        }
        let mut message = [0; 109];
        message[..77].copy_from_slice(&self.owner_prefix);
        message[77..].copy_from_slice(&secret);
        let owner: [u8; 64] = Sha3_512::digest(message).into();
        (owner == opening[73..137]).then_some(message)
    }
}
