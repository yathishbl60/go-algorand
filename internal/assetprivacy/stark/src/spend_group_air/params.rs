// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 16384;
pub const TRANSFER_PUBLIC_VALUES: usize = 264;
pub const UNSHIELD_PUBLIC_VALUES: usize = 196;
pub(super) const CARRY: usize = 256;
pub(super) const WORK: usize = CARRY + 3;
pub const WIDTH: usize = WORK + crate::spendable_note_air::WIDTH;
pub(super) const INPUT_ROWS: usize = crate::spendable_note_air::FRAME_ROWS;
pub(super) const OUTPUT_ROW: usize = 2 * INPUT_ROWS;
pub(super) const OUTPUT_ROWS: usize = crate::MIN_TRACE_ROWS;
pub(super) const FLAGS: usize = crate::spendable_note_air::PREPROCESSED;
pub const PREPROCESSED: usize = FLAGS + 16;

const _: () = assert!(INPUT_ROWS == 7356);
const _: () = assert!(OUTPUT_ROWS == 512);
const _: () = assert!(OUTPUT_ROW + 2 * OUTPUT_ROWS <= ROWS);
