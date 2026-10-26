// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Public native domain, selected from the finalized body's current policy.
#[derive(Clone, Copy)]
pub struct Domain {
    pub network: [u8; 32],
    pub asset: u64,
    pub policy: [u8; 64],
}

impl Domain {
    pub(super) fn bytes(self) -> [u8; 104] {
        let mut bytes = [0; 104];
        bytes[..32].copy_from_slice(&self.network);
        bytes[32..40].copy_from_slice(&self.asset.to_be_bytes());
        bytes[40..].copy_from_slice(&self.policy);
        bytes
    }
}
