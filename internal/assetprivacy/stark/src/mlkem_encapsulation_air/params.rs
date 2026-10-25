// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 65536;
pub(crate) const FRAME_ROWS: usize = 40585;
pub const PUBLIC_VALUES: usize = crate::mlkem_pke_air::PUBLIC_VALUES;
pub(super) const RHO: usize = crate::mlkem_pke_air::WIDTH;
pub(super) const KEY: usize = RHO + 256;
pub(super) const COINS: usize = KEY + 256;
pub(super) const WORK: usize = COINS + 256;
pub(super) const WORK_WIDTH: usize = crate::mlkem_stream_derivation_air::WIDTH;
pub const WIDTH: usize = WORK + WORK_WIDTH;
pub(super) const CHILD_PRE: usize = 1967;
pub(super) const STAGES: usize = 18;
pub const PREPROCESSED: usize = CHILD_PRE + 4 * STAGES;
pub(super) const MATRIX_ROW: usize = 1024;
pub(super) const NOISE_ROW: usize = MATRIX_ROW + 9 * crate::mlkem_stream_matrix_air::FRAME_ROWS;
pub(super) const PKE_ROW: usize = NOISE_ROW + 7 * 512;
pub(super) const END_ROW: usize = PKE_ROW + crate::mlkem_pke_air::ROWS;

const _: () = assert!(WORK_WIDTH >= crate::mlkem_stream_matrix_air::WIDTH);
const _: () = assert!(WORK_WIDTH >= 5522);
const _: () = assert!(crate::mlkem_stream_derivation_air::ROWS == MATRIX_ROW);
const _: () = assert!(crate::mlkem_stream_matrix_air::ROWS == 4096);
const _: () = assert!(END_ROW == FRAME_ROWS);
const _: () = assert!(FRAME_ROWS <= ROWS);
const _: () = assert!(crate::mlkem_pke_air::WIDTH == 9328);

const _: () = assert!(crate::mlkem_stream_matrix_air::FRAME_ROWS == 2177);
