// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Proof-pipeline regressions; these do not execute native transactions or nodes.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/owned_notes/mod.rs"]
mod support;

/// Serial execution bounds overlap between the six actual proof generations.
#[test]
fn ownership_proof_pipeline_accepts_valid_inputs_and_rejects_corruption() {
    support::malformed::reject();
    for (count, rows) in [(1, 512), (1, 1024), (2, 1024)] {
        support::pipeline::round_trip(count, rows);
    }
    support::private_links::reject();
}
