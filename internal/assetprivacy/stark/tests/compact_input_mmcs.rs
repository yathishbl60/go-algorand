// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Input-MMCS backend interoperability; no complete PCS or native transaction claim.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/input_mmcs/mod.rs"]
mod support;

#[test]
fn coefficient_input_mmcs_matches_stock_verification_and_preserves_dense_fallback() {
    support::streamed::compare(&[1]);
    support::streamed::compare(&[1, 2]);
    let sixteen: Vec<_> = (0..16).map(|index| index % 3 + 1).collect();
    support::streamed::compare(&sixteen);
    support::dense::compare();
    support::malformed::reject();
}
