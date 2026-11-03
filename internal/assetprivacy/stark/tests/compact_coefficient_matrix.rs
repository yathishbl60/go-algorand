// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Honest matrix and query-gather backend integration, not native node coverage.
#![cfg(feature = "native-privacy-compact")]

#[path = "support/coefficient_matrix/mod.rs"]
mod support;

#[test]
fn coefficient_backed_matrices_match_dense_bit_reversed_evaluations() {
    support::modes::compare(512, 1, 11, 3);
    support::modes::compare(1024, 4, 13, 11);
    support::modes::ownership();
    support::malformed::reject();
}
