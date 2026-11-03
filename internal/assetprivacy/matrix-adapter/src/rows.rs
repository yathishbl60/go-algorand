// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Safe source contract: dimensions and checked owned rows, with optional fast cells.
/// Returned row width is validated by the bridge before unchecked trait access.
pub trait CheckedRows<T: Clone + Send + Sync>: Send + Sync {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn checked_row(&self, row: usize) -> Option<Vec<T>>;

    fn checked_get(&self, row: usize, column: usize) -> Option<T> {
        self.checked_row(row)?.get(column).cloned()
    }
}
