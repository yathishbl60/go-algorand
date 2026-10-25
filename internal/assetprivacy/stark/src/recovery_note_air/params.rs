// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 4096;
pub const PUBLIC_VALUES: usize = 64;
pub(super) const ACCOUNT: usize = 0;
pub(super) const VALUE: usize = 256;
pub(super) const SERIAL: usize = 320;
pub(super) const OWNER: usize = 576;
pub(super) const COMMIT: usize = 608;
pub(super) const WORK: usize = 640;
pub(super) const SCRATCH: usize = 7350;
pub const WIDTH: usize = WORK + SCRATCH;
pub(super) const FLAGS: usize = crate::membership_vertical::PREPROCESSED;
pub const PREPROCESSED: usize = FLAGS + 12;
pub(super) const NOTE_ROWS: usize = 512;
pub(super) const NULLIFIER_ROW: usize = NOTE_ROWS;
pub(super) const NULLIFIER_ROWS: usize = 97;
pub(super) const MEMBER_ROW: usize = NULLIFIER_ROW + NULLIFIER_ROWS;
pub const FRAME_ROWS: usize = MEMBER_ROW + crate::membership_vertical::FRAME_ROWS;

const _: () = assert!(FRAME_ROWS == 3714);
const _: () = assert!(FRAME_ROWS <= ROWS);
const _: () = assert!(crate::membership_vertical::WIDTH <= SCRATCH);
