// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{mlkem_dot_air as dot, mlkem_keygen_ntt_air as core};

pub const ROWS: usize = 16384;
pub(super) const CORE_ROWS: usize = 10240;
pub(super) const CORE_WIDTH: usize = 9362;
pub(super) const CORE_PRE: usize = 570;
pub(super) const DOT_ROWS: usize = 2048;
pub(super) const DOT_WIDTH: usize = 6161;
pub(super) const DOT_PRE: usize = 145;
pub(super) const FLAGS: usize = CORE_PRE + DOT_PRE;
pub(super) const CARRY: usize = DOT_WIDTH;
pub(super) const TRANSFORMED: usize = 7826;
pub(super) const GROUP: usize = CORE_WIDTH;
pub const WIDTH: usize = GROUP + 32;
pub const PREPROCESSED: usize = FLAGS + 8;
pub const PUBLIC_VALUES: usize = 1184 + 32;
const _: () = assert!(core::WIDTH == CORE_WIDTH && core::PREPROCESSED == CORE_PRE);
const _: () =
    assert!(dot::WIDTH == DOT_WIDTH && dot::ROWS == DOT_ROWS && dot::SCHEDULE_WIDTH == DOT_PRE);
const _: () = assert!(CARRY + 768 < TRANSFORMED && CORE_ROWS + DOT_ROWS <= ROWS);
