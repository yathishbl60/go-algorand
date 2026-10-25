// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 65536;
pub const WIDTH: usize = 24869;
pub const PUBLIC_VALUES: usize = 1132;
pub(super) const RHO: usize = 768;
pub(super) const VIEW_EPOCH: usize = 1024;
pub(super) const ACCOUNT: usize = 1088;
pub(super) const OWNER: usize = 1344;
pub(super) const SPEND_EPOCH: usize = 1856;
pub(super) const VALUE: usize = 1920;
pub(super) const WORK: usize = 1984;
pub(super) const CHILD_PRE: usize = crate::envelope_air::PREPROCESSED;
pub const PREPROCESSED: usize = CHILD_PRE + 16;
pub(super) const VIEW_ROW: usize = crate::envelope_air::FRAME_ROWS;
pub(super) const VIEW_ROWS: usize =
    crate::view_key_air::ROWS + crate::membership_vertical::FRAME_ROWS;
pub(super) const SPEND_ROW: usize = VIEW_ROW + VIEW_ROWS;
pub(super) const SPEND_ROWS: usize =
    crate::spend_key_air::ROWS + crate::membership_vertical::FRAME_ROWS;
pub(super) const ELIGIBILITY_ROW: usize = SPEND_ROW + SPEND_ROWS;
pub(super) const ELIGIBILITY_ROWS: usize = crate::current_eligibility_air::FRAME_ROWS;
pub(super) const END_ROW: usize = ELIGIBILITY_ROW + ELIGIBILITY_ROWS;
const _: () = assert!(END_ROW <= ROWS);
const _: () = assert!(WORK + crate::envelope_air::WIDTH == WIDTH);
const _: () = assert!(crate::registered_key_air::WIDTH <= crate::envelope_air::WIDTH);
const _: () = assert!(crate::current_eligibility_air::WIDTH <= crate::envelope_air::WIDTH);
const _: () = assert!(crate::current_eligibility_air::PREPROCESSED <= CHILD_PRE);
const _: () = assert!(END_ROW == 59245);
