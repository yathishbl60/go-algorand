// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemStreamMatrixAir, ROWS, WIDTH};
use crate::{Value, mlkem_sample_air::MlKemSampleAir, shake128_matrix_air::Shake128MatrixAir};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) struct Prepared {
    pub seed: [u8; 32],
    pub states: Vec<[u64; 25]>,
    pub prefix: Vec<u8>,
    pub coefficients: [u16; 256],
}

impl MlKemStreamMatrixAir {
    /// Generates the private relation; a 16-block prefix may be exhausted.
    pub fn trace(&self, seed: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let input = self.prepare(seed)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_rows(&mut values, (WIDTH, 0, 0), input);
        Some(RowMajorMatrix::new(values, WIDTH))
    }
    pub(super) fn prepare(&self, seed: [u8; 32]) -> Option<Prepared> {
        let shake = Shake128MatrixAir::new(self.first, self.second, 16)?;
        let (states, prefix) = shake.stream_inputs(seed);
        let coefficients = MlKemSampleAir::new(16)?.values(&prefix)?;
        Some(Prepared { seed, states, prefix, coefficients })
    }
}
