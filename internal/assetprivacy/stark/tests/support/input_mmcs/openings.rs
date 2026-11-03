// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, V, corruption, frontier, verify};
use p3_commit::{BatchOpeningRef, Mmcs};

pub fn multi(fixture: &Fixture, indices: &[usize]) {
    let (values, proof) = fixture
        .input
        .try_open_multi(indices, &fixture.data)
        .expect("valid gather");
    assert_eq!(values.len(), indices.len());
    for (rows, &index) in values.iter().zip(indices) {
        expected_rows(fixture, index, rows);
    }
    assert!(verify::accepts(fixture, indices, &values, &proof));
    if !indices.is_empty() {
        corruption::reject(fixture, indices, &values, &proof);
    }
    if indices.len() == 1024 {
        assert!(proof.1.sibling_hashes.is_empty());
        frontier::compare(fixture, &values, &proof);
    }
}

fn expected_rows(fixture: &Fixture, index: usize, rows: &[Vec<V>]) {
    assert_eq!(rows.len(), fixture.expected.len());
    for (row, matrix) in rows.iter().zip(&fixture.expected) {
        assert_eq!(
            row,
            &matrix.values[index * matrix.width..(index + 1) * matrix.width]
        );
    }
}

pub fn single(fixture: &Fixture) {
    for index in [0, 7, 1023] {
        let opening = fixture
            .input
            .try_open_batch(index, &fixture.data)
            .expect("single opening");
        expected_rows(fixture, index, &opening.opened_values);
        verify_single(fixture, index, &opening);
    }
}

fn verify_single(
    fixture: &Fixture,
    index: usize,
    opening: &p3_commit::BatchOpening<V, super::InputMmcs>,
) {
    assert!(
        fixture
            .input
            .verify_batch(&fixture.cap, &fixture.dimensions, index, opening.into())
            .is_ok()
    );
    let borrowed = BatchOpeningRef::new(&opening.opened_values, &opening.opening_proof);
    assert!(
        fixture
            .oracle
            .verify_batch(&fixture.cap, &fixture.dimensions, index, borrowed)
            .is_ok()
    );
}
