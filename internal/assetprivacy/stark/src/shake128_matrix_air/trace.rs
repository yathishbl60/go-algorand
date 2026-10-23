// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OUTPUT, SEED, Shake128MatrixAir};
use crate::Value;
use core::ops::Range;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{NUM_KECCAK_COLS, generate_trace_rows};
use p3_matrix::dense::RowMajorMatrix;

impl Shake128MatrixAir {
    /// Generates private trace data only; no sampling or proof evaluation occurs.
    pub fn trace(&self, seed: [u8; 32]) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(self.rows * self.trace_width());
        self.write_into(seed, &mut values, self.trace_width(), 0)
            .expect("fixed SHAKE128 shape");
        RowMajorMatrix::new(values, self.trace_width())
    }

    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        row_width: usize,
        start: usize,
    ) -> Option<Vec<u8>> {
        let end = start.checked_add(self.trace_width())?;
        if values.len() != self.rows.checked_mul(row_width)? || end > row_width {
            return None;
        }
        let (inputs, output) = self.inputs(seed);
        let keccak = generate_trace_rows(inputs, 0);
        write_rows(values, row_width, start..end, &keccak, &seed, &output);
        Some(output)
    }
}

fn write_rows(
    values: &mut [Value],
    width: usize,
    columns: Range<usize>,
    keccak: &RowMajorMatrix<Value>,
    seed: &[u8; 32],
    output: &[u8],
) {
    for (index, parent) in values.chunks_exact_mut(width).enumerate() {
        let row = &mut parent[columns.clone()];
        row[..NUM_KECCAK_COLS]
            .copy_from_slice(&keccak.values[index * NUM_KECCAK_COLS..][..NUM_KECCAK_COLS]);
        bits(&mut row[SEED..OUTPUT], seed);
        bits(&mut row[OUTPUT..], output);
    }
}

fn bits(row: &mut [Value], input: &[u8]) {
    for (target, &source) in row.as_chunks_mut::<8>().0.iter_mut().zip(input) {
        for (bit, value) in target.iter_mut().enumerate() {
            *value = Value::from_bool(source & (1 << bit) != 0);
        }
    }
}
