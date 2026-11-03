// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, Proof, V, Values, verify};
use algorand_asset_privacy_stark::compact_storage::TreeCap;
use p3_commit::Mmcs;
use p3_field::PrimeCharacteristicRing;

pub fn reject(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) {
    let mut changed = values.clone();
    changed[0][0][0] += V::ONE;
    verify::rejects(fixture, indices, &changed, proof);
    let mut salts = proof.clone();
    salts.0[0][0][0] += V::ONE;
    verify::rejects(fixture, indices, values, &salts);
    let mut wrong_indices = indices.to_vec();
    wrong_indices[0] ^= 1;
    verify::rejects(fixture, &wrong_indices, values, proof);
    if indices.len() > 1 {
        let mut reordered = indices.to_vec();
        reordered.swap(0, 1);
        verify::rejects(fixture, &reordered, values, proof);
    }
    if !proof.1.sibling_hashes.is_empty() {
        siblings(fixture, indices, values, proof);
    }
    context(fixture, indices, values, proof);
}

fn siblings(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) {
    let mut changed = proof.clone();
    changed.1.sibling_hashes[0][0] ^= 1;
    verify::rejects(fixture, indices, values, &changed);
    let mut missing = proof.clone();
    missing.1.sibling_hashes.pop();
    verify::rejects(fixture, indices, values, &missing);
    let mut extra = proof.clone();
    extra.1.sibling_hashes.push([0; 8]);
    verify::rejects(fixture, indices, values, &extra);
}

fn context(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) {
    let mut roots = fixture.cap.roots().to_vec();
    roots[0][0] ^= 1;
    let cap = TreeCap::new(roots);
    assert!(
        fixture
            .oracle
            .verify_multi_batch(&cap, &fixture.dimensions, indices, values, proof)
            .is_err()
    );
    assert!(
        fixture
            .input
            .verify_multi_batch(&cap, &fixture.dimensions, indices, values, proof)
            .is_err()
    );
    wrong_dimensions(fixture, indices, values, proof);
}

fn wrong_dimensions(fixture: &Fixture, indices: &[usize], values: &Values, proof: &Proof) {
    let mut dimensions = fixture.dimensions.clone();
    dimensions[0].width += 1;
    assert!(
        fixture
            .input
            .verify_multi_batch(&fixture.cap, &dimensions, indices, values, proof)
            .is_err()
    );
    assert!(
        fixture
            .oracle
            .verify_multi_batch(&fixture.cap, &dimensions, indices, values, proof)
            .is_err()
    );
}
