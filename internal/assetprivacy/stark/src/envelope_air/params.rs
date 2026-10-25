// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 65536;
pub const WIDTH: usize = 22885;
pub const PUBLIC_VALUES: usize = 1068;
pub(super) const RHO: usize = 768;
pub(super) const SHARED: usize = 1024;
pub(super) const AAD: usize = 1280;
pub(super) const KEY: usize = 1792;
pub(super) const OPENING: usize = 2048;
pub(super) const COMMIT: usize = 6144;
pub(super) const EPOCH: usize = 6176;
pub(super) const NONZERO: usize = EPOCH + 64;
pub(super) const WORK: usize = NONZERO + 1;
pub(super) const WORK_WIDTH: usize = WIDTH - WORK;
pub(super) const CHILD_PRE: usize = crate::mlkem_encapsulation_air::PREPROCESSED;
pub const PREPROCESSED: usize = CHILD_PRE + 20;
pub(super) const AAD_ROW: usize = crate::mlkem_encapsulation_air::FRAME_ROWS;
pub(super) const HKDF_ROW: usize = AAD_ROW + 1024;
pub(super) const GCM_ROW: usize = HKDF_ROW + 512;
pub(super) const NOTE_ROW: usize = GCM_ROW + crate::gcm_vertical::FRAME_ROWS;
pub(super) const END_ROW: usize = NOTE_ROW + 512;
pub(crate) const FRAME_ROWS: usize = END_ROW;

const _: () = assert!(END_ROW <= ROWS);
const _: () = assert!(crate::mlkem_encapsulation_air::WIDTH <= WORK_WIDTH);
const _: () = assert!(crate::gcm_vertical::WIDTH <= WORK_WIDTH);
const _: () = assert!(crate::envelope_aad_air::WIDTH <= WORK_WIDTH);
const _: () = assert!(crate::envelope_aad_air::ROWS == 1024);

const _: () = assert!(FRAME_ROWS == 47882);
