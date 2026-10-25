// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 4096;
pub const PUBLIC_VALUES: usize = 32;
pub(super) const ACCOUNT: usize = 0;
pub(super) const REVISION: usize = 256;
pub(super) const UPDATED: usize = 320;
pub(super) const LEAF: usize = 384;
pub(super) const WORK: usize = 416;
pub const WIDTH: usize = WORK + crate::membership_vertical::WIDTH;
pub(super) const FLAGS: usize = crate::membership_vertical::PREPROCESSED;
pub const PREPROCESSED: usize = FLAGS + 8;
pub(super) const MEMBER_ROW: usize = crate::eligible_record_air::ROWS;
pub const FRAME_ROWS: usize = MEMBER_ROW + crate::membership_vertical::FRAME_ROWS;

const _: () = assert!(crate::eligible_record_air::WIDTH <= WIDTH - WORK);
const _: () = assert!(FRAME_ROWS == 3617);
const _: () = assert!(FRAME_ROWS <= ROWS);
