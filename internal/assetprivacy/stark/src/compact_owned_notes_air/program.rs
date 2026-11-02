// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    HASH_HEADER, HASH_PROGRAM, HEAD, HOLD_OWNER, NOTE_PROGRAM, OwnedNotesAir, PERIODIC, SELECT,
    WORD_ACTIVE, WORD_INDEX,
};
use p3_air::BaseAir;
use p3_field::PrimeField64;

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn build_program(&self) -> Option<Vec<Vec<F>>> {
        let mut columns = empty_program(self.rows)?;
        let note = self.note.preprocessed_trace()?;
        let hash = self.hash.preprocessed_trace()?;
        for row in 0..self.rows {
            for column in 0..9 {
                columns[NOTE_PROGRAM + column][row] = note.values[row * 9 + column];
            }
            for column in 0..11 {
                columns[HASH_PROGRAM + column][row] = hash.values[row * 11 + column];
            }
            self.mark_program_row(&mut columns, row);
        }
        Some(columns)
    }

    fn mark_program_row(&self, columns: &mut [Vec<F>], row: usize) {
        if let Some(position) = self.note.position(row) {
            let offset = position.byte_offset();
            columns[HEAD][row] = F::from_bool(offset == 0);
            columns[HOLD_OWNER][row] = F::from_bool(offset < 511);
            if (74..=136).contains(&offset) && offset % 2 == 0 {
                let word = (offset - 74) / 2;
                columns[WORD_ACTIVE][row] = F::ONE;
                columns[WORD_INDEX][row] = F::from_usize(word);
                columns[SELECT + word][row] = F::ONE;
            }
        }
        if let Some(position) = self.hash.position(row) {
            columns[HASH_HEADER][row] = F::from_bool(position.is_first());
        }
    }
}

fn empty_program<F: PrimeField64>(rows: usize) -> Option<Vec<Vec<F>>> {
    let mut columns = Vec::with_capacity(PERIODIC);
    for _ in 0..PERIODIC {
        let mut column = Vec::new();
        column.try_reserve_exact(rows).ok()?;
        column.resize(rows, F::ZERO);
        columns.push(column);
    }
    Some(columns)
}
