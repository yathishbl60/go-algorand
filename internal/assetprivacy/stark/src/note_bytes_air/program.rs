// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{NoteBytesAir, PREPROCESSED, validation};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

/// Fixed preprocessing columns; future buses must use this trusted row program.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum ProgramColumn {
    ExpectedByte,
    PublicByte,
    NonzeroActive,
    NonzeroEnd,
    Dummy,
    NoteIndex,
    ByteOffset,
    ByteActive,
    WordActive,
}

impl<F: PrimeField64> NoteBytesAir<F> {
    pub(super) fn program(&self) -> RowMajorMatrix<F> {
        let mut values = Vec::with_capacity(self.rows() * PREPROCESSED);
        for row in 0..self.rows() {
            values.extend(self.program_row(row));
        }
        RowMajorMatrix::new(values, PREPROCESSED)
    }

    fn program_row(&self, row: usize) -> [F; PREPROCESSED] {
        let Some(position) = self.position(row) else {
            return dummy();
        };
        let offset = position.byte_offset();
        let expected = self.expected_byte(offset);
        [
            F::from_u8(expected.unwrap_or(0)),
            F::from_bool(expected.is_some()),
            F::from_bool(validation::nonzero_active(offset)),
            F::from_bool(validation::nonzero_end(offset)),
            F::ZERO,
            F::from_usize(position.note_index()),
            F::from_usize(offset),
            F::ONE,
            F::from_bool(offset != 0),
        ]
    }
}

fn dummy<F: PrimeField64>() -> [F; PREPROCESSED] {
    let mut row = [F::ZERO; PREPROCESSED];
    row[ProgramColumn::PublicByte as usize] = F::ONE;
    row[ProgramColumn::Dummy as usize] = F::ONE;
    row
}
