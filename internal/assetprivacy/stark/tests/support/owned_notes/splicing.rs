// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, Trace, fixture};
use algorand_asset_privacy_stark::note_bytes_air::NoteBytesAir;
use p3_air::BaseAir;

pub fn trace(air: &Air, original: &Trace, notes: &[[u8; 512]], case: usize) -> Trace {
    let mut changed = original.clone();
    if case == 0 {
        replace_hashes(air, &mut changed);
    } else {
        replace_note_owners(air, &mut changed, notes, case);
    }
    changed
}

fn replace_hashes(air: &Air, changed: &mut Trace) {
    let (notes, secrets) = fixture::inputs(2, 17);
    let other = air
        .trace(&notes, &secrets)
        .expect("alternate valid ownership");
    let width = air.width();
    // Keep original canonical notes/owner bridge; substitute the valid hash region.
    for row in 0..air.rows() {
        let start = row * width;
        changed.values[start + 11..start + 1931]
            .copy_from_slice(&other.values[start + 11..start + 1931]);
        changed.values[start + 1964] = other.values[start + 1964];
    }
}

fn replace_note_owners(air: &Air, changed: &mut Trace, notes: &[[u8; 512]], case: usize) {
    let mut notes = notes.to_vec();
    if case == 1 {
        notes[0][73..77].rotate_left(2);
    } else {
        let (first, rest) = notes.split_at_mut(1);
        first[0][73..137].swap_with_slice(&mut rest[0][73..137]);
    }
    let note = NoteBytesAir::new(7, [19; 64], 2)
        .and_then(|component| component.trace(&notes))
        .expect("canonical altered notes");
    let width = air.width();
    for row in 0..air.rows() {
        changed.values[row * width..row * width + 11]
            .copy_from_slice(&note.values[row * 11..row * 11 + 11]);
    }
}
