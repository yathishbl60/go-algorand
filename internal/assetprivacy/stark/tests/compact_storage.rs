// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Differential backend regressions, not native transaction end-to-end tests.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/storage/mod.rs"]
mod support;

#[test]
fn compact_storage_matches_stock_masking_transforms_and_quotients() {
    support::masking::compare();
    support::quotient::compare();
}
