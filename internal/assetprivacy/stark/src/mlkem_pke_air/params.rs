// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 16384;
pub const WIDTH: usize = 9328;
pub const PUBLIC_VALUES: usize = 544;
pub(super) const RHAT: usize = 3072;
pub(super) const DOT: usize = 3840;
pub(super) const WORK: usize = 4864;
pub(super) const NOISE: usize = 6161;
pub(super) const MESSAGE: usize = 7953;
pub(super) const FINAL: usize = 8209;
pub(super) const GLOBAL_END: usize = 9233;
pub(super) const DOT_ROW: usize = 3072;
pub(super) const INVERSE_ROW: usize = 5120;
pub(super) const CODEC_ROW: usize = 14336;
pub(super) const INV_SPAN: usize = 2304;
pub(super) const CONTROL: usize = 1683;
pub(super) const FWD_FLAGS: usize = CONTROL;
pub(super) const INV_FLAGS: usize = CONTROL + 4;
pub(super) const DOT_FLAGS: usize = CONTROL + 8;
pub(super) const CODEC_FLAGS: usize = CONTROL + 12;
pub(super) const FWD_SLOT: usize = CONTROL + 16;
pub(super) const INV_SLOT: usize = CONTROL + 19;
pub(super) const ADD_SLOT: usize = CONTROL + 23;
pub(super) const ADD_INDEX: usize = CONTROL + 27;
pub(super) const RAW_CARRY: usize = CONTROL + 283;
pub(super) const PREPROCESSED: usize = CONTROL + 284;

// The compiled frame layout is part of the candidate proof profile.
const _: () = assert!(crate::mlkem_ntt_air::WIDTH == 920);
const _: () = assert!(crate::mlkem_ntt_air::SCHEDULE_WIDTH == 514);
const _: () = assert!(crate::mlkem_dot_air::WIDTH == 6161);
const _: () = assert!(crate::mlkem_dot_air::SCHEDULE_WIDTH == 145);
const _: () = assert!(crate::mlkem_ciphertext_air::WIDTH == 1119);
const _: () = assert!(CODEC_ROW + crate::mlkem_ciphertext_air::ROWS == ROWS);

const _: () = assert!(crate::mlkem_ntt_air::MlKemNttAir::forward().trace_rows() == 1024);
const _: () = assert!(crate::mlkem_ntt_air::MlKemNttAir::inverse().trace_rows() == 2048);
const _: () = assert!(crate::mlkem_dot_air::ROWS == 2048);
const _: () = assert!(crate::mlkem_ciphertext_air::COEFFICIENTS == 1024);
const _: () = assert!(crate::mlkem_ciphertext_air::PUBLIC_VALUES == PUBLIC_VALUES);
