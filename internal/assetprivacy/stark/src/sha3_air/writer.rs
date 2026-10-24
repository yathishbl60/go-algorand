// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Sha3Air;
use crate::Value;
use core::ops::Range;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{NUM_KECCAK_COLS, NUM_ROUNDS, generate_trace_rows};
use p3_matrix::dense::RowMajorMatrix;

impl Sha3Air {
    pub(crate) fn write_into(
        &self,
        message: &[u8],
        values: &mut [Value],
        row_width: usize,
        start: usize,
    ) -> Option<()> {
        let columns = self.target(message.len(), values.len(), row_width, start)?;
        let keccak = generate_trace_rows(self.permutation_inputs(message), 0);
        self.write_rows(values, (row_width, columns), message, &keccak);
        Some(())
    }

    fn target(&self, bytes: usize, len: usize, width: usize, start: usize) -> Option<Range<usize>> {
        let end = start.checked_add(self.width())?;
        if bytes != self.message_len
            || end > width
            || len != self.trace_rows().checked_mul(width)?
        {
            return None;
        }
        Some(start..end)
    }

    fn write_rows(
        &self,
        values: &mut [Value],
        layout: (usize, Range<usize>),
        message: &[u8],
        keccak: &RowMajorMatrix<Value>,
    ) {
        for (index, parent) in values.chunks_exact_mut(layout.0).enumerate() {
            let row = &mut parent[layout.1.clone()];
            row.fill(Value::ZERO);
            row[..NUM_KECCAK_COLS]
                .copy_from_slice(&keccak.values[index * NUM_KECCAK_COLS..][..NUM_KECCAK_COLS]);
            write_message(row, message);
            if index / NUM_ROUNDS < self.blocks() {
                row[self.block_offset() + index / NUM_ROUNDS] = Value::ONE;
            }
        }
    }
}

fn write_message(row: &mut [Value], message: &[u8]) {
    for (i, byte) in message.iter().enumerate() {
        for bit in 0..8 {
            row[NUM_KECCAK_COLS + i * 8 + bit] = Value::from_bool(byte & (1 << bit) != 0);
        }
    }
}
