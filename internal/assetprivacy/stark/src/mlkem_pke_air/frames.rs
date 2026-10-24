// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CODEC_FLAGS, DOT_FLAGS, FINAL, FWD_FLAGS, INV_FLAGS, WORK};
use crate::gcm_vertical::frame::Frame;

pub(super) fn ntt(inverse: bool) -> Frame {
    Frame {
        columns: WORK..WORK + 920,
        preprocessing: 0..514,
        public: 0..0,
        flags: if inverse { INV_FLAGS } else { FWD_FLAGS },
    }
}
pub(super) fn dot() -> Frame {
    Frame {
        columns: 0..6161,
        preprocessing: 514..659,
        public: 0..0,
        flags: DOT_FLAGS,
    }
}
pub(super) fn codec() -> Frame {
    Frame {
        columns: FINAL..FINAL + 1119,
        preprocessing: 659..1683,
        public: 0..544,
        flags: CODEC_FLAGS,
    }
}
