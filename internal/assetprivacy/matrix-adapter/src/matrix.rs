// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CheckedRows, SafeMatrix};
use p3_matrix::Matrix;

impl<M: CheckedRows<T>, T: Clone + Send + Sync> Matrix<T> for SafeMatrix<M, T> {
    fn width(&self) -> usize {
        self.inner.width()
    }

    fn height(&self) -> usize {
        self.inner.height()
    }

    fn get(&self, row: usize, column: usize) -> Option<T> {
        if row >= self.inner.height() || column >= self.inner.width() {
            return None;
        }
        self.inner.checked_get(row, column)
    }

    // P3 requires this unsafe signature to terminate its circular default methods.
    // The body uses only safe bounds/width checks; invalid callers panic, not UB.
    unsafe fn row_unchecked(
        &self,
        row: usize,
    ) -> impl IntoIterator<Item = T, IntoIter = impl Iterator<Item = T> + Send + Sync> {
        assert!(row < self.inner.height(), "matrix row out of bounds");
        let values = self.inner.checked_row(row).expect("matrix row unavailable");
        assert_eq!(
            values.len(),
            self.inner.width(),
            "matrix row width mismatch"
        );
        values
    }
}
