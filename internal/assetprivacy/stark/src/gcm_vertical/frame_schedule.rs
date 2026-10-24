// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FRAME_ROWS, GH_FLAGS, GcmVerticalAir, PREPROCESSED};
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl GcmVerticalAir {
    /// Compact frame retains every GHASH row and one final carry row.
    pub(crate) fn frame_preprocessed_trace(&self) -> RowMajorMatrix<Value> {
        let mut table = self.preprocessed_trace().expect("fixed GCM schedule");
        table.values.truncate(FRAME_ROWS * PREPROCESSED);
        let last = &mut table.values[(FRAME_ROWS - 1) * PREPROCESSED..];
        last[GH_FLAGS + 2] = Value::ONE;
        last[GH_FLAGS + 3] = Value::ZERO;
        table
    }
}
