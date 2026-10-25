// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AuthorizedRecipientAir, ELIGIBILITY_ROW, RecipientInput, SPEND_ROW, VIEW_ROW, WIDTH, WORK,
};
use crate::Value;

impl AuthorizedRecipientAir {
    pub(super) fn write_authorization(
        &self,
        input: &RecipientInput<'_>,
        keys: [u8; 64],
        eligibility: [u8; 64],
        values: &mut [Value],
    ) -> Option<()> {
        for (air, record, first) in [
            (&self.view, input.view(), VIEW_ROW),
            (&self.spend, input.spend(), SPEND_ROW),
        ] {
            air.write_into(record, keys, values, (WIDTH, WORK, first))?;
        }
        let shape = (WIDTH, WORK, ELIGIBILITY_ROW);
        self.eligibility
            .write_into(input.eligibility(), eligibility, values, shape)?;
        Some(())
    }
}
