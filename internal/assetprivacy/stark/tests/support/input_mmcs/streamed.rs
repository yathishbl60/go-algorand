// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, fixture, openings};
use p3_commit::Mmcs;
use p3_matrix::Matrix;

pub fn compare(widths: &[usize]) {
    let fixture = fixture::new(widths);
    metadata(&fixture);
    openings::multi(&fixture, &[]);
    openings::multi(&fixture, &[1023, 0, 1, 7, 1023, 7]);
    openings::multi(&fixture, &[0, 1, 2, 3]);
    openings::single(&fixture);
    if widths.len() == 1 {
        let all: Vec<_> = (0..1024).collect();
        openings::multi(&fixture, &all);
    }
}

fn metadata(fixture: &Fixture) {
    let matrices = fixture.input.get_matrices(&fixture.data);
    assert_eq!(matrices.len(), fixture.expected.len());
    for (matrix, expected) in matrices.into_iter().zip(&fixture.expected) {
        assert_eq!(
            (matrix.height(), matrix.width()),
            (expected.height(), expected.width())
        );
        assert_eq!(matrix.get(7, 0), expected.get(7, 0));
        assert_eq!(
            matrix.row_slice(1023).expect("honest LDE row").as_ref(),
            expected.row_slice(1023).expect("dense row").as_ref()
        );
    }
}
