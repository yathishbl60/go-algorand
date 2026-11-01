// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, PREPROCESSED};
use crate::serial_keccak::{RC_POSITIONS, ROUND_CONSTANTS};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

/// Seven Iota bit columns start immediately after RoundIndex.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum ProgramColumn {
    Continuation,
    Dummy,
    PermutationIndex,
    RoundIndex,
    RoundConstant0,
}

impl<F: PrimeField64> CompactKeccakAir<F> {
    pub(super) fn program(&self) -> RowMajorMatrix<F> {
        let mut values = Vec::with_capacity(self.rows() * PREPROCESSED);
        for row in 0..self.rows() {
            values.extend(self.program_row(row));
        }
        RowMajorMatrix::new(values, PREPROCESSED)
    }

    fn program_row(&self, index: usize) -> [F; PREPROCESSED] {
        let mut row = [F::ZERO; PREPROCESSED];
        let Some(position) = self.position(index) else {
            row[ProgramColumn::Dummy as usize] = F::ONE;
            return row;
        };
        row[ProgramColumn::Continuation as usize] = F::from_bool(!position.is_last());
        row[ProgramColumn::PermutationIndex as usize] = F::from_usize(position.permutation_index());
        row[ProgramColumn::RoundIndex as usize] = F::from_usize(position.round_index());
        let constant = ROUND_CONSTANTS[position.round_index()];
        for (column, bit) in RC_POSITIONS.into_iter().enumerate() {
            row[ProgramColumn::RoundConstant0 as usize + column] =
                F::from_bool(constant & (1u64 << bit) != 0);
        }
        row
    }
}
