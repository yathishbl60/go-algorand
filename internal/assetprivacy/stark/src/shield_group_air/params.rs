// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 1024;
pub const PUBLIC_VALUES: usize = 100;
pub const CONFIDENTIAL_PUBLIC_VALUES: usize = PUBLIC_VALUES + 128;
pub const PREPROCESSED: usize = 8;
pub(super) const NOTE_WIDTH: usize = 7350;
pub(super) const CARRY: usize = 256;
pub(super) const GROUP: usize = CARRY + 3;
pub(super) const WORK: usize = GROUP + 32;
pub const WIDTH: usize = WORK + NOTE_WIDTH;
const _: () = assert!(crate::MIN_TRACE_ROWS == 512);
const _: () = assert!(p3_keccak_air::NUM_KECCAK_COLS + 588 * 8 + 9 + 4 == NOTE_WIDTH);
