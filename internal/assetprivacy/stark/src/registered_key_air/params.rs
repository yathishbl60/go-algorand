// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 8192;
pub const WIDTH: usize = 15768;
pub const PUBLIC_VALUES: usize = 32;
pub const PREPROCESSED: usize = 776;
pub(super) const ACCOUNT: usize = 768;
pub(super) const EPOCH: usize = 1024;
pub(super) const CREATED: usize = 1088;
pub(super) const RHO: usize = 1152;
pub(super) const OWNER: usize = 1408;
pub(super) const LEAF: usize = 1920;
pub(super) const WORK: usize = 1952;
pub(super) const FLAGS: usize = 768;

const _: () = assert!(WIDTH == WORK + crate::view_key_air::WIDTH);
const _: () = assert!(crate::spend_key_air::WIDTH <= WIDTH - WORK);
const _: () = assert!(crate::membership_vertical::WIDTH <= WIDTH - WORK);
const _: () = assert!(crate::view_key_air::PREPROCESSED <= FLAGS);
const _: () = assert!(crate::membership_vertical::PREPROCESSED <= FLAGS);
const _: () = assert!(PREPROCESSED == FLAGS + 8);
const _: () = assert!(crate::view_key_air::ROWS + crate::membership_vertical::FRAME_ROWS <= ROWS);
const _: () = assert!(crate::spend_key_air::ROWS + crate::membership_vertical::FRAME_ROWS <= ROWS);
