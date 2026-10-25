// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CurrentEligibilityAir, EligibilityInput};

pub(super) struct Prepared<'a> {
    pub input: EligibilityInput<'a>,
    pub leaf: [u8; 64],
}

impl<'a> Prepared<'a> {
    pub fn new(
        air: &CurrentEligibilityAir,
        input: EligibilityInput<'a>,
        root: [u8; 64],
    ) -> Option<Self> {
        let leaf = air
            .record
            .commitment(input.account, input.revision, input.updated)?;
        if root == [0; 64] || air.membership.root(leaf, input.path) != root {
            return None;
        }
        Some(Self { input, leaf })
    }
}
