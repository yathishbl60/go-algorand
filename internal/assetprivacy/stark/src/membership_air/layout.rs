// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir};

impl MembershipAir {
    pub(super) fn stride(&self) -> usize {
        self.hash.width() + 33
    }
    pub(super) fn stage_start(&self, level: usize) -> usize {
        level * self.stride()
    }
    pub(super) fn digest_start(&self, level: usize) -> usize {
        self.stage_start(level) + self.hash.width()
    }
    pub(super) fn direction_column(&self, level: usize) -> usize {
        self.digest_start(level) + 32
    }
    pub(crate) fn leaf_start(&self) -> usize {
        DEPTH * self.stride()
    }
    pub(crate) fn width(&self) -> usize {
        self.leaf_start() + 32
    }
    pub fn leaf_limbs<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == self.width()).then(|| &row[self.leaf_start()..])
    }
}
