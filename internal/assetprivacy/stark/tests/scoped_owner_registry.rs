// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Artifact-bound Owner proof pipeline, not a native transaction/node scenario.
#![cfg(feature = "native-privacy")]

#[path = "support/scoped_owner/mod.rs"]
mod support;

#[test]
fn scoped_owner_artifact_pipeline_accepts_real_proof_and_rejects_corruption() {
    let fixture = support::fixture::new();
    support::registry::reject(&fixture);
    support::proof::round_trip(&fixture);
}
