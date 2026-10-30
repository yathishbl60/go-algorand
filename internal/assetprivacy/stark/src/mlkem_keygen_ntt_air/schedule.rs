// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    FLAGS, FRAME_ROWS, MlKemKeygenNttAir, NOISE_ROWS, NOISE_SCHEDULE, NTT_ROWS, PREPROCESSED, ROWS,
};
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemKeygenNttAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        self.schedule_rows(ROWS)
    }

    pub(crate) fn frame_preprocessed_trace(&self) -> RowMajorMatrix<Value> {
        self.schedule_rows(FRAME_ROWS)
    }

    fn schedule_rows(&self, rows: usize) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(rows * PREPROCESSED);
        copy_schedule(
            &mut values,
            &self.noise.preprocessed_trace().expect("noise schedule"),
            0,
            0,
        );
        flags(&mut values, 0, NOISE_ROWS, 0);
        let ntt = self.ntt.preprocessed_trace().expect("NTT schedule");
        for i in 0..6 {
            let first = NOISE_ROWS + i * NTT_ROWS;
            copy_schedule(&mut values, &ntt, first, NOISE_SCHEDULE);
            flags(&mut values, first, NTT_ROWS, i + 1);
        }
        RowMajorMatrix::new(values, PREPROCESSED)
    }
}

fn copy_schedule(values: &mut [Value], child: &RowMajorMatrix<Value>, first: usize, column: usize) {
    for (i, row) in child.values.chunks_exact(child.width).enumerate() {
        let start = (first + i) * PREPROCESSED + column;
        values[start..start + child.width].copy_from_slice(row);
    }
}

fn flags(values: &mut [Value], first: usize, rows: usize, stage: usize) {
    for local in 0..rows {
        let start = (first + local) * PREPROCESSED + FLAGS + 4 * stage;
        values[start] = Value::ONE;
        values[start + 1] = Value::from_bool(local == 0);
        values[start + 2] = Value::from_bool(local + 1 == rows);
        values[start + 3] = Value::from_bool(local + 1 < rows);
    }
}
