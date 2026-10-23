// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemMatrixAir;
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl MlKemMatrixAir {
    pub(super) fn schedule(&self) -> Option<RowMajorMatrix<Value>> {
        let xof = self.xof.preprocessed_trace()?;
        let sample = self.sample.preprocessed_trace()?;
        if xof.height() != sample.height() {
            return None;
        }
        let width = xof.width + sample.width;
        let mut values = Value::zero_vec(self.trace_rows() * width);
        for (index, row) in values.chunks_exact_mut(width).enumerate() {
            row[..xof.width]
                .copy_from_slice(&xof.values[index * xof.width..(index + 1) * xof.width]);
            row[xof.width..]
                .copy_from_slice(&sample.values[index * sample.width..(index + 1) * sample.width]);
        }
        Some(RowMajorMatrix::new(values, width))
    }
}
