// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{current_eligibility_air::EligibilityInput, membership_air::MembershipPath};

#[derive(Clone, Copy)]
pub struct SpendInput<'a> {
    pub opening: &'a [u8; 512],
    pub secret: [u8; 32],
    pub note_path: &'a MembershipPath,
    pub eligibility_revision: u64,
    pub eligibility_updated: u64,
    pub eligibility_path: &'a MembershipPath,
}

impl<'a> SpendInput<'a> {
    pub(super) fn eligibility(&self) -> EligibilityInput<'a> {
        EligibilityInput {
            account: self.opening[137..169].try_into().expect("fixed account"),
            revision: self.eligibility_revision,
            updated: self.eligibility_updated,
            path: self.eligibility_path,
        }
    }
    pub(super) fn value(&self) -> u64 {
        u64::from_be_bytes(self.opening[169..177].try_into().expect("fixed value"))
    }
}
