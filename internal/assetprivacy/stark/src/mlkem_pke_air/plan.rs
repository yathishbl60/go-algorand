// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CODEC_ROW, DOT_ROW, INV_SPAN, INVERSE_ROW};

pub(super) enum Step {
    Forward(usize, usize),
    Dot(usize),
    Inverse(usize, usize),
    Addition(usize, usize),
    Codec(usize),
}

pub(super) fn at(row: usize) -> Step {
    if row < DOT_ROW {
        return Step::Forward(row / 1024, row % 1024);
    }
    if row < INVERSE_ROW {
        return Step::Dot(row - DOT_ROW);
    }
    if row < CODEC_ROW {
        let slot = (row - INVERSE_ROW) / INV_SPAN;
        let local = (row - INVERSE_ROW) % INV_SPAN;
        return if local < 2048 {
            Step::Inverse(slot, local)
        } else {
            Step::Addition(slot, local - 2048)
        };
    }
    Step::Codec(row - CODEC_ROW)
}
