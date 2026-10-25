// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DOMAIN, MESSAGE_BYTES, ViewKeyAir};

pub(super) fn prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 155] {
    let mut prefix = [0; 155];
    prefix[..50].copy_from_slice(DOMAIN);
    prefix[50] = 1;
    prefix[51..83].copy_from_slice(&network);
    prefix[83..91].copy_from_slice(&asset.to_be_bytes());
    prefix[91..155].copy_from_slice(&policy);
    prefix
}

impl ViewKeyAir {
    pub(super) fn message(
        &self,
        key: &[u8],
        account: [u8; 32],
        epoch: u64,
        created: u64,
    ) -> Option<[u8; MESSAGE_BYTES]> {
        if key.len() != 1184 || account == [0; 32] || epoch == 0 {
            return None;
        }
        let mut message = [0; MESSAGE_BYTES];
        message[..155].copy_from_slice(&self.prefix);
        message[155..187].copy_from_slice(&account);
        message[187] = self.role;
        message[188..196].copy_from_slice(&epoch.to_be_bytes());
        message[196..204].copy_from_slice(&created.to_be_bytes());
        message[204..].copy_from_slice(key);
        Some(message)
    }
}
