// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, AuthorizedRecipientAir, CHILD_PRE, OWNER, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl AuthorizedRecipientAir {
    pub(super) fn eval_eligibility_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + crate::current_eligibility_air::WIDTH];
        let account = crate::current_eligibility_air::wires(child)
            .expect("eligibility wires")
            .account_bits;
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 12];
        for (&root, &current) in row[ACCOUNT..OWNER].iter().zip(account) {
            builder.when(flag).assert_eq(root, current);
        }
    }
}
