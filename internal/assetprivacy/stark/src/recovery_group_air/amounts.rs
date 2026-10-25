// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, POSITIVE, RecoveryGroupAir, binding::bits};
use crate::{
    Value,
    amounts::{self, Amounts},
};
use p3_air::{AirBuilder, WindowAccess};

impl RecoveryGroupAir {
    pub(super) fn eval_amounts<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let values = core::array::from_fn(|index| bits(row, index));
        let carry = row[CARRY..POSITIVE].try_into().expect("carry width");
        amounts::eval(
            builder,
            &Amounts { values, carry },
            self.operation.conversion(),
        );
        for (index, required) in [self.operation.private(), self.operation.partial()]
            .into_iter()
            .enumerate()
        {
            if required {
                let sum: AB::Expr = bits(row, index + 2).into_iter().map(Into::into).sum();
                builder.assert_one(sum * row[POSITIVE + index]);
            }
        }
    }
}
