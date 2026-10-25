// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::membership_air::MembershipPath;

#[derive(Clone, Copy)]
pub struct RecoveryInput<'a> {
    pub opening: &'a [u8; 512],
    pub path: &'a MembershipPath,
}

impl RecoveryInput<'_> {
    pub(super) fn value(&self) -> u64 {
        u64::from_be_bytes(self.opening[169..177].try_into().expect("fixed note value"))
    }
}
