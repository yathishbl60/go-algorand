// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg(feature = "native-privacy-compact")]

#[path = "support/coefficient_pcs/mod.rs"]
mod support;

#[test]
fn coefficient_pcs_stock_interoperability() {
    support::direct::run();
    support::guards::run();
    support::concurrent::run();
    support::notes::run();
}
