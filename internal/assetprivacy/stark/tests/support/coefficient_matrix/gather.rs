// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Mask, V};
use algorand_asset_privacy_stark::compact_storage::CoefficientMatrix;
use p3_dft::Radix2Dit;

pub fn compare(matrix: &CoefficientMatrix<&Mask>, dense: &Dense) {
    let indices = [matrix.height() - 1, 0, 7, 1, matrix.height() - 1, 7, 0];
    for tile in [1, 3, matrix.width() + 17] {
        let budget = matrix.height() * tile.min(matrix.width());
        let output = indices.len() * matrix.width();
        let rows = matrix
            .gather(&indices, tile, budget, output, &Radix2Dit::default())
            .expect("bounded original-order gather");
        check_rows(&rows, &indices, dense, matrix.width());
        check_budgets(matrix, &indices, tile, budget, output);
    }
}

fn check_rows(rows: &[Vec<V>], indices: &[usize], dense: &Dense, width: usize) {
    for (actual, &index) in rows.iter().zip(indices) {
        assert_eq!(actual, &dense.values[index * width..(index + 1) * width]);
    }
    assert_eq!(rows[0], rows[4]);
    assert_eq!(rows[2], rows[5]);
}

fn check_budgets(
    matrix: &CoefficientMatrix<&Mask>,
    indices: &[usize],
    tile: usize,
    budget: usize,
    output: usize,
) {
    let dft = Radix2Dit::default();
    assert!(
        matrix
            .gather(indices, tile, budget - 1, output, &dft)
            .is_none()
    );
    assert!(
        matrix
            .gather(indices, tile, budget, output - 1, &dft)
            .is_none()
    );
}
