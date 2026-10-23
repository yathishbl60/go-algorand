// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MemberNoteAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MemberNoteAir {
    pub(super) fn eval_commitment<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let note = self
            .note
            .commitment_limbs(&row[..self.note.width()])
            .expect("note width");
        let leaf = self
            .membership
            .leaf_limbs(&row[self.note.width()..])
            .expect("membership width");
        for limb in 0..32 {
            builder.assert_eq(note[limb], leaf[limb]);
        }
    }
}
