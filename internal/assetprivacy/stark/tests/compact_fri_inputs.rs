// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Backend FRI integration only; no native transaction or complete PCS is exercised.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/fri_inputs/mod.rs"]
mod support;

#[test]
fn coefficient_backed_fri_authenticates_inputs_and_rejects_corruption() {
    let fixture = support::fixture::new();
    let argument = support::proving::prove(&fixture, 17);
    assert!(support::verification::accepts(&fixture, &argument, 17));
    support::negative::reject(&fixture, argument);
}
