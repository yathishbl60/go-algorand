// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, PREPROCESSED};
use crate::Value;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn copy(
    values: &mut [Value],
    shape: (usize, usize, usize),
    table: &RowMajorMatrix<Value>,
) {
    assert!(table.width() <= FLAGS, "spend note schedule width");
    for local in 0..table.height() {
        let row = &mut values[(shape.2 + local) * shape.0 + shape.1..][..PREPROCESSED];
        row[..table.width()]
            .copy_from_slice(&table.values[local * table.width()..][..table.width()]);
    }
}
