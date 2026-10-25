// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, WORK};
use crate::gcm_vertical::frame::Frame;
use core::ops::Range;

pub(super) fn component(stage: usize, width: usize, pre: usize, public: Range<usize>) -> Frame {
    Frame {
        columns: WORK..WORK + width,
        preprocessing: 0..pre,
        public,
        flags: FLAGS + 4 * stage,
    }
}
