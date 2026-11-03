// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Mask, fixture, gather, generic};
use algorand_asset_privacy_stark::compact_storage::CoefficientMatrix;
use std::sync::Arc;

pub fn compare(rows: usize, width: usize, log: usize, shift: u8) {
    let mask = fixture::masked(rows, width);
    let domain = fixture::domain(log, shift);
    let dense = fixture::dense(&mask, domain);
    let matrix = CoefficientMatrix::new(&mask, domain).expect("borrowed virtual LDE");
    assert_eq!(
        (matrix.height(), matrix.width()),
        (domain.size(), width + 10)
    );
    assert!(matrix.height() > matrix.mask().coefficient_rows());
    direct(&matrix, &dense);
    gather::compare(&matrix, &dense);
    let bridge = matrix.into_matrix().expect("checked P3 matrix bridge");
    generic::compare(&bridge, &dense);
    gather::compare(bridge.inner(), &dense);
}

fn direct(matrix: &CoefficientMatrix<&Mask>, dense: &Dense) {
    for row in [0, 1, 7, matrix.height() / 2 + 3, matrix.height() - 1] {
        assert_eq!(
            matrix.row(row).expect("actual row"),
            dense.values[row * matrix.width()..(row + 1) * matrix.width()]
        );
        for column in 0..matrix.width() {
            assert_eq!(
                matrix.get(row, column),
                Some(dense.values[row * matrix.width() + column])
            );
        }
    }
}

pub fn ownership() {
    let domain = fixture::domain(11, 7);
    let shared: Arc<Mask> = Arc::new(fixture::masked(512, 2));
    let expected = fixture::dense(&shared, domain);
    let matrix = CoefficientMatrix::new(Arc::clone(&shared), domain).expect("Arc-owned source");
    drop(shared);
    let matrix = matrix.into_matrix().expect("Arc source bridge");
    generic::compare(&matrix, &expected);
    let owned = CoefficientMatrix::new(fixture::masked(512, 2), domain).expect("owned source");
    generic::compare(&owned.into_matrix().expect("owned bridge"), &expected);
}
