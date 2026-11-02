// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Backend differential integration, not native transaction end-to-end coverage.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/streamed_tree/mod.rs"]
mod support;

#[test]
fn streamed_commitments_and_paths_match_pinned_hiding_mmcs() {
    support::cases::compare(&[1], 10, 3);
    support::cases::compare(&[2], 11, 11);
    support::cases::compare(&[1, 2], 10, 3);
    support::cases::compare(&[1, 1], 11, 7);
    let sixteen: Vec<_> = (0..16).map(|index| index % 3 + 1).collect();
    support::cases::compare(&sixteen, 10, 11);
    support::malformed::reject();
}
