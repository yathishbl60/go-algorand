// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AES_FLAGS, AES_SCHEDULE, AES_WIDTH, GH_FLAGS, GH_START};
use core::ops::Range;

pub(super) struct Frame {
    pub columns: Range<usize>,
    pub preprocessing: Range<usize>,
    pub public: Range<usize>,
    pub flags: usize,
}

pub(super) fn aes() -> Frame {
    Frame {
        columns: 0..AES_WIDTH,
        preprocessing: 0..AES_SCHEDULE,
        public: 0..268,
        flags: AES_FLAGS,
    }
}

pub(super) fn ghash() -> Frame {
    Frame {
        columns: GH_START..GH_START + 1537,
        preprocessing: 61..226,
        public: 12..268,
        flags: GH_FLAGS,
    }
}
