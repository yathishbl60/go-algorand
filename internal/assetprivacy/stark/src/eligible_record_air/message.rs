// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, DOMAIN, EligibleRecordAir, MESSAGE_BYTES};

pub(super) fn prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; ACCOUNT] {
    let mut prefix = [0; ACCOUNT];
    prefix[..45].copy_from_slice(DOMAIN);
    prefix[45] = 1;
    prefix[46..78].copy_from_slice(&network);
    prefix[78..86].copy_from_slice(&asset.to_be_bytes());
    prefix[86..150].copy_from_slice(&policy);
    prefix
}

impl EligibleRecordAir {
    pub(super) fn message(
        &self,
        account: [u8; 32],
        revision: u64,
        updated: u64,
    ) -> Option<[u8; MESSAGE_BYTES]> {
        if account == [0; 32] || revision == 0 {
            return None;
        }
        let mut message = [0; MESSAGE_BYTES];
        message[..150].copy_from_slice(&self.prefix);
        message[150..182].copy_from_slice(&account);
        message[182..190].copy_from_slice(&revision.to_be_bytes());
        message[190..198].copy_from_slice(&updated.to_be_bytes());
        Some(message) // The canonical Frozen byte198 remains zero.
    }
}
