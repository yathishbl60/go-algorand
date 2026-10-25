// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LEAF, MembershipVerticalAir, WIDTH};

impl MembershipVerticalAir {
    /// Private leaf limbs for the enclosing note or key-authorization relation.
    pub fn leaf_limbs<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[LEAF..LEAF + 32])
    }
}
