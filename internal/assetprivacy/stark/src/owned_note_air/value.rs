// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::OwnedNoteAir;

impl OwnedNoteAir {
    pub(crate) fn serial_bit_column(&self, bit: usize) -> usize {
        self.note.opening_bit_column(177 + bit / 8, bit % 8)
    }

    /// Shares the committed private value with the conservation component.
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        if row.len() != self.width() {
            return None;
        }
        self.note.value_bits(&row[..self.note.width()])
    }

    /// Private commitment limbs for the membership relation; never publish them.
    pub fn commitment_limbs<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        if row.len() != self.width() {
            return None;
        }
        Some(&row[self.digest_offset()..self.digest_offset() + 32])
    }
}
