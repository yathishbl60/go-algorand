// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::NullifiedNoteAir;

impl NullifiedNoteAir {
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        if row.len() != self.width() {
            return None;
        }
        self.owned.value_bits(&row[..self.owned.width()])
    }

    /// Private commitment for membership; the public value is only the nullifier.
    pub fn commitment_limbs<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        if row.len() != self.width() {
            return None;
        }
        self.owned.commitment_limbs(&row[..self.owned.width()])
    }
}
