// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, WORK};
use crate::gcm_vertical::frame::Frame;

pub(super) fn component(stage: usize, width: usize, preprocessed: usize) -> Frame {
    Frame {
        columns: WORK..WORK + width,
        preprocessing: 0..preprocessed,
        public: 0..0,
        flags: CHILD_PRE + 4 * stage,
    }
}

pub(super) fn pke() -> Frame {
    Frame {
        columns: 0..crate::mlkem_pke_air::WIDTH,
        preprocessing: 0..CHILD_PRE,
        public: 0..crate::mlkem_pke_air::PUBLIC_VALUES,
        flags: CHILD_PRE + 4 * 17,
    }
}
