// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Borrow, CoefficientMatrix, MaskedTrace, Value32};
use algorand_asset_privacy_matrix_adapter::{CheckedRows, SafeMatrix};

impl<S: Borrow<MaskedTrace<Value32>> + Send + Sync> CoefficientMatrix<S> {
    /// Move this honest source into P3's Matrix interface without copying coefficients.
    pub fn into_matrix(self) -> Option<SafeMatrix<Self, Value32>> {
        SafeMatrix::new(self)
    }
}

impl<S: Borrow<MaskedTrace<Value32>> + Send + Sync> CheckedRows<Value32> for CoefficientMatrix<S> {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn checked_row(&self, row: usize) -> Option<Vec<Value32>> {
        self.row(row)
    }

    fn checked_get(&self, row: usize, column: usize) -> Option<Value32> {
        self.get(row, column)
    }
}
