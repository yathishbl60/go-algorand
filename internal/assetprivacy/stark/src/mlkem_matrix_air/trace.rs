// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemMatrixAir;
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemMatrixAir {
    /// Uses the same SHAKE and first-accepted witness math without a field trace.
    pub fn values(&self, seed: [u8; 32]) -> Option<[u16; 256]> {
        self.sample.values(&self.xof.prefix(seed))
    }

    /// Generates one composed private trace; returns None on bounded-prefix exhaustion.
    pub fn trace(&self, seed: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let width = <Self as BaseAir<Value>>::width(self);
        let mut values = Value::zero_vec(self.trace_rows() * width);
        let prefix = self.xof.write_into(seed, &mut values, width, 0)?;
        self.sample
            .write_into(&prefix, &mut values, width, self.sampler_start())?;
        Some(RowMajorMatrix::new(values, width))
    }
}
