// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MEMBER_ROW, NULLIFIER_ROW, NULLIFIER_ROWS, RecoveryNoteAir, WORK, prepared::Prepared};
use crate::Value;

pub(super) fn write(
    air: &RecoveryNoteAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let child = (shape.0, shape.1 + WORK, shape.2);
    air.note.write_into(input.input.opening, values, child)?;
    let child = (shape.0, shape.1 + WORK, shape.2 + NULLIFIER_ROW);
    air.nullifier
        .write_frame(&input.message, values, child, NULLIFIER_ROWS)?;
    let child = (shape.0, shape.1 + WORK, shape.2 + MEMBER_ROW);
    air.membership
        .write_into(input.commitment, input.input.path, values, child)?;
    Some(())
}
