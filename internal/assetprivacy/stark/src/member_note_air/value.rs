// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MemberNoteAir;

impl MemberNoteAir {
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        if row.len() != self.width() {
            return None;
        }
        self.note.value_bits(&row[..self.note.width()])
    }
}
