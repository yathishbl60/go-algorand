// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ENCODED_SIZE, NoteBytesAir};
use p3_field::PrimeField64;

/// Immutable logical address derived from the constructor-owned row program.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RowPosition {
    note: usize,
    byte: usize,
}

impl RowPosition {
    pub fn note_index(self) -> usize {
        self.note
    }
    pub fn byte_offset(self) -> usize {
        self.byte
    }
    /// Offset of the low byte in previous + 256 * current; absent at note start.
    pub fn overlapping_word_offset(self) -> Option<usize> {
        self.byte.checked_sub(1)
    }
}

impl<F: PrimeField64> NoteBytesAir<F> {
    /// Padding and out-of-range rows have no note/byte/word identity.
    pub fn position(&self, row: usize) -> Option<RowPosition> {
        (row < self.count * ENCODED_SIZE).then_some(RowPosition {
            note: row / ENCODED_SIZE,
            byte: row % ENCODED_SIZE,
        })
    }
}
