// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 65536;
pub const WIDTH: usize = 28069;
pub const PUBLIC_VALUES: usize = 1132;
pub(super) const RHO: usize = 768;
pub(super) const EPOCH: usize = 1024;
pub(super) const OPENING: usize = 1088;
pub(super) const WORK: usize = 5184;
pub(super) const CHILD_PRE: usize = crate::envelope_air::PREPROCESSED;
pub const PREPROCESSED: usize = CHILD_PRE + 8;
pub(super) const VIEW_ROW: usize = crate::envelope_air::FRAME_ROWS;
pub(super) const VIEW_ROWS: usize =
    crate::view_key_air::ROWS + crate::membership_vertical::FRAME_ROWS;
pub const FRAME_ROWS: usize = VIEW_ROW + VIEW_ROWS;

const _: () = assert!(FRAME_ROWS == 52011);
const _: () = assert!(FRAME_ROWS <= ROWS);
const _: () = assert!(WORK + crate::envelope_air::WIDTH == WIDTH);
const _: () = assert!(crate::registered_key_air::WIDTH <= WIDTH - WORK);
const _: () = assert!(crate::registered_key_air::PREPROCESSED <= CHILD_PRE);
