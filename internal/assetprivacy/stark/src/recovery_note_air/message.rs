// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::RecoveryNoteAir;

pub(super) fn prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 147] {
    let mut prefix = [0; 147];
    prefix[..43].copy_from_slice(b"Algorand/AssetPrivacy/Candidate1/Nullifier/");
    prefix[43..75].copy_from_slice(&network);
    prefix[75..83].copy_from_slice(&asset.to_be_bytes());
    prefix[83..].copy_from_slice(&policy);
    prefix
}

impl RecoveryNoteAir {
    pub(super) fn message(&self, opening: &[u8; 512]) -> [u8; 179] {
        let mut message = [0; 179];
        message[..147].copy_from_slice(&self.prefix);
        message[147..].copy_from_slice(&opening[177..209]);
        message
    }
}
