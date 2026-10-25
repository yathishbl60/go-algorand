// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ELIGIBILITY_ROW, MEMBER_ROW, NULLIFIER_ROW, NULLIFIER_ROWS, OWNER_ROW, OWNER_ROWS,
    SpendableNoteAir, WORK, prepared::Prepared,
};
use crate::Value;

pub(super) fn write(
    air: &SpendableNoteAir,
    input: &Prepared<'_>,
    eligibility_root: [u8; 64],
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let child = (shape.0, shape.1 + WORK, shape.2);
    air.note.write_into(input.input.opening, values, child)?;
    let child = (shape.0, shape.1 + WORK, shape.2 + OWNER_ROW);
    air.owner
        .write_frame(&input.owner_message, values, child, OWNER_ROWS)?;
    let child = (shape.0, shape.1 + WORK, shape.2 + NULLIFIER_ROW);
    air.nullifier
        .write_frame(&input.nullifier_message, values, child, NULLIFIER_ROWS)?;
    write_membership(air, input, eligibility_root, values, shape)
}

fn write_membership(
    air: &SpendableNoteAir,
    input: &Prepared<'_>,
    eligibility_root: [u8; 64],
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let child = (shape.0, shape.1 + WORK, shape.2 + MEMBER_ROW);
    air.membership
        .write_into(input.commitment, input.input.note_path, values, child)?;
    let child = (shape.0, shape.1 + WORK, shape.2 + ELIGIBILITY_ROW);
    air.eligibility
        .write_into(input.input.eligibility(), eligibility_root, values, child)
}
