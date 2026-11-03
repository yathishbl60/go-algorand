// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, Proof, Values};
use p3_commit::Mmcs;

pub fn accepts(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) -> bool {
    fixture
        .oracle
        .verify_multi_batch(&fixture.cap, &fixture.dimensions, indices, values, proof)
        .is_ok()
        && fixture
            .input
            .verify_multi_batch(&fixture.cap, &fixture.dimensions, indices, values, proof)
            .is_ok()
}

pub fn rejects(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) {
    assert!(
        fixture
            .oracle
            .verify_multi_batch(&fixture.cap, &fixture.dimensions, indices, values, proof)
            .is_err()
    );
    assert!(
        fixture
            .input
            .verify_multi_batch(&fixture.cap, &fixture.dimensions, indices, values, proof)
            .is_err()
    );
}
