// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::HmacAir;

impl HmacAir {
    pub(super) fn digest_start(&self) -> usize {
        self.inner.width() + self.outer.width()
    }
    pub(super) fn key_start(&self) -> usize {
        self.digest_start() + 64
    }
    pub(super) fn output_start(&self) -> usize {
        self.key_start() + 512
    }
    pub(crate) fn width(&self) -> usize {
        self.output_start() + 512
    }
}
