// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Sha3Air;
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{NUM_KECCAK_COLS, NUM_ROUNDS, generate_trace_rows};
use p3_matrix::dense::RowMajorMatrix;

impl Sha3Air {
    /// A trusted parent may omit unused trailing rows after every flag clears.
    pub(crate) fn write_frame(
        &self,
        message: &[u8],
        values: &mut [Value],
        target: (usize, usize, usize),
        rows: usize,
    ) -> Option<()> {
        self.check_frame(message.len(), values.len(), target, rows)?;
        let keccak = generate_trace_rows(self.permutation_inputs(message), 0);
        self.write_frame_rows(message, values, target, rows, &keccak);
        Some(())
    }

    fn check_frame(
        &self,
        bytes: usize,
        len: usize,
        target: (usize, usize, usize),
        rows: usize,
    ) -> Option<()> {
        let (stride, column, first) = target;
        if bytes != self.message_len
            || stride == 0
            || !len.is_multiple_of(stride)
            || column.checked_add(self.width())? > stride
            || first.checked_add(rows)? > len / stride
            || rows < self.blocks() * NUM_ROUNDS + 1
            || rows > self.trace_rows()
        {
            return None;
        }
        Some(())
    }

    fn write_frame_rows(
        &self,
        message: &[u8],
        values: &mut [Value],
        target: (usize, usize, usize),
        rows: usize,
        keccak: &RowMajorMatrix<Value>,
    ) {
        let targets = values.chunks_exact_mut(target.0).skip(target.2).take(rows);
        for (index, parent) in targets.enumerate() {
            let row = &mut parent[target.1..target.1 + self.width()];
            row.fill(Value::ZERO);
            row[..NUM_KECCAK_COLS]
                .copy_from_slice(&keccak.values[index * NUM_KECCAK_COLS..][..NUM_KECCAK_COLS]);
            write_message(&mut row[NUM_KECCAK_COLS..self.block_offset()], message);
            if index / NUM_ROUNDS < self.blocks() {
                row[self.block_offset() + index / NUM_ROUNDS] = Value::ONE;
            }
        }
    }
}

fn write_message(row: &mut [Value], message: &[u8]) {
    for (bit, value) in row.iter_mut().enumerate() {
        *value = Value::from_bool(message[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
