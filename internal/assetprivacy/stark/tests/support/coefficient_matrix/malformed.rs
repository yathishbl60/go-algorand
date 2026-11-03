// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Mask, V, fixture};
use algorand_asset_privacy_stark::compact_storage::CoefficientMatrix;
use p3_dft::Radix2Dit;

pub fn reject() {
    let mask = fixture::masked(512, 1);
    assert!(CoefficientMatrix::new(&mask, fixture::domain(9, 3)).is_none());
    let matrix = CoefficientMatrix::new(&mask, fixture::domain(11, 3)).expect("valid source");
    assert!(matrix.get(matrix.height(), 0).is_none());
    assert!(matrix.get(0, matrix.width()).is_none());
    assert!(matrix.row(matrix.height()).is_none());
    reject_gather(&matrix);
}

fn reject_gather(matrix: &CoefficientMatrix<&Mask>) {
    let dft = Radix2Dit::default();
    assert!(
        matrix
            .gather(&[matrix.height()], 1, usize::MAX, usize::MAX, &dft)
            .is_none()
    );
    assert!(
        matrix
            .gather(&[0], 0, usize::MAX, usize::MAX, &dft)
            .is_none()
    );
    assert_eq!(
        matrix.gather(&[], 1, 0, 0, &dft),
        Some(Vec::<Vec<V>>::new())
    );
    assert!(matrix.gather(&[], 0, 0, 0, &dft).is_none());
}
