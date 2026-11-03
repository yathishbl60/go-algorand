// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, V};
use p3_matrix::Matrix;

pub fn compare(matrix: &impl Matrix<V>, dense: &Dense) {
    assert_eq!(
        (matrix.height(), matrix.width()),
        (dense.height(), dense.width())
    );
    for row in [0, 1, 7, matrix.height() / 2 + 3, matrix.height() - 1] {
        compare_row(matrix, dense, row);
    }
    assert!(matrix.get(matrix.height(), 0).is_none());
    assert!(matrix.get(0, matrix.width()).is_none());
    assert!(matrix.row(matrix.height()).is_none());
    assert!(matrix.row_slice(matrix.height()).is_none());
    let wrapped = matrix.wrapping_row_slices(matrix.height() - 1, 2);
    assert_eq!(wrapped[1].as_ref(), &dense.values[..dense.width()]);
}

fn compare_row(matrix: &impl Matrix<V>, dense: &Dense, row: usize) {
    let expected = &dense.values[row * dense.width()..(row + 1) * dense.width()];
    let actual: Vec<_> = matrix.row(row).expect("generic row").into_iter().collect();
    assert_eq!(actual, expected);
    assert_eq!(
        matrix.row_slice(row).expect("generic slice").as_ref(),
        expected
    );
    for (column, &value) in expected.iter().enumerate() {
        assert_eq!(matrix.get(row, column), Some(value));
    }
    // This safe stock method exercises both row_subseq_unchecked default paths.
    let (packed, suffix) = matrix.horizontally_packed_row::<V>(row);
    assert_eq!(packed.chain(suffix).collect::<Vec<_>>(), expected);
}
