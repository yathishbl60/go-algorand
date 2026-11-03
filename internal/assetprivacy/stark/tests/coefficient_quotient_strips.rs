// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg(feature = "native-privacy-compact")]

#[path = "support/coefficient_quotient_strips/mod.rs"]
mod support;

#[test]
fn strip_quotient_owned_notes_stock_verification() {
    support::notes::run();
}

#[test]
#[ignore = "full-height bounded validation; run explicitly"]
fn strip_quotient_owned_notes_full_height_stock_verification() {
    support::notes::full_height();
}
