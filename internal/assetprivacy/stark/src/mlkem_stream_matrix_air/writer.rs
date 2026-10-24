// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, BLOCK_ROWS, MlKemStreamMatrixAir, ROWS, SEED, WIDTH, trace::Prepared};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::generate_trace_rows;

impl MlKemStreamMatrixAir {
    /// Writes a bounded frame, preparing exhaustion before changing any cell.
    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<[u16; 256]> {
        let (stride, start, first) = target;
        if stride == 0
            || !values.len().is_multiple_of(stride)
            || start.checked_add(WIDTH)? > stride
            || first.checked_add(ROWS)? > values.len() / stride
        {
            return None;
        }
        let input = self.prepare(seed)?;
        let output = input.coefficients;
        self.write_rows(values, target, input);
        Some(output)
    }
    pub(super) fn write_rows(
        &self,
        values: &mut [Value],
        target: (usize, usize, usize),
        input: Prepared,
    ) {
        let mut cursor = 0;
        for block in 0..16 {
            let keccak = generate_trace_rows(vec![input.states[block]], 0);
            for offset in 0..BLOCK_ROWS {
                let row = &mut values
                    [(target.2 + block * BLOCK_ROWS + offset) * target.0 + target.1..][..WIDTH];
                copy_keccak(row, &keccak.values, offset);
                cursor = self.write_row(row, &input, (block, offset, cursor));
            }
        }
        self.write_padding(values, target, &input, cursor);
    }
    fn write_padding(
        &self,
        values: &mut [Value],
        target: (usize, usize, usize),
        input: &Prepared,
        cursor: usize,
    ) {
        let (stride, start, first) = target;
        for index in ACTIVE_ROWS..ROWS {
            let row = &mut values[(first + index) * stride + start..][..WIDTH];
            row.fill(Value::ZERO);
            self.write_row(row, input, (15, BLOCK_ROWS, cursor));
        }
    }
}

fn copy_keccak(row: &mut [Value], keccak: &[Value], offset: usize) {
    row.fill(Value::ZERO);
    if offset < 24 {
        row[..SEED].copy_from_slice(&keccak[offset * SEED..][..SEED]);
    }
}
