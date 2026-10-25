// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub(super) fn prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 149] {
    crate::nullifier_v2::prefix(network, asset, policy)
}
