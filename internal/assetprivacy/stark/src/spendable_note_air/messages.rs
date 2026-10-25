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

pub(super) fn nullifier_prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 147] {
    let mut prefix = [0; 147];
    prefix[..43].copy_from_slice(b"Algorand/AssetPrivacy/Candidate1/Nullifier/");
    prefix[43..75].copy_from_slice(&network);
    prefix[75..83].copy_from_slice(&asset.to_be_bytes());
    prefix[83..].copy_from_slice(&policy);
    prefix
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
    pub(super) fn nullifier_message(&self, opening: &[u8; 512]) -> [u8; 179] {
        let mut message = [0; 179];
        message[..147].copy_from_slice(&self.nullifier_prefix);
        message[147..].copy_from_slice(&opening[177..209]);
        message
    }
}
