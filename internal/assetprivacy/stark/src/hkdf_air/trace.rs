// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::HkdfAir;
use crate::{MIN_TRACE_ROWS, Value};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl HkdfAir {
    pub fn trace(&self, shared: &[u8; 32], salt: &[u8; 64]) -> Option<RowMajorMatrix<Value>> {
        let mut values = Value::zero_vec(MIN_TRACE_ROWS * self.width());
        self.write_into(shared, salt, &mut values, (self.width(), 0, 0))?;
        Some(RowMajorMatrix::new(values, self.width()))
    }
}
