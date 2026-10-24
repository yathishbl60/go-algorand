// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncapsulationAir, RHO, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl MlKemEncapsulationAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for index in (0..4864).chain(6161..9233).chain(RHO..WORK) {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[index], main.next_slice()[index]);
        }
        for index in RHO..WORK {
            builder.assert_bool(main.current_slice()[index]);
        }
    }
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "fixed private binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
