// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 8192;
pub(super) const SOURCE_ACCOUNT: usize = 256;
pub(super) const SOURCE_OWNER: usize = 512;
pub(super) const CARRY: usize = 544;
pub(super) const POSITIVE: usize = 547;
pub(super) const WORK: usize = 549;
pub const WIDTH: usize = WORK + crate::recovery_note_air::WIDTH;
pub(super) const INPUT_ROWS: usize = crate::recovery_note_air::FRAME_ROWS;
pub(super) const OUTPUT_ROWS: usize = 512;
pub(super) const FLAGS: usize = crate::recovery_note_air::PREPROCESSED;
pub const PREPROCESSED: usize = FLAGS + 12;

const _: () = assert!(INPUT_ROWS == 3714);
const _: () = assert!(INPUT_ROWS + 2 * OUTPUT_ROWS <= ROWS);
