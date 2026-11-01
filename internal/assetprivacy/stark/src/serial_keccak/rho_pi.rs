// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{STATE_BITS, index::rho_source};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

pub(super) fn eval<AB: AirBuilder<F = Value>>(builder: &mut AB, offset: usize, pre: usize) {
    let main = builder.main();
    let (local, next) = (main.current_slice(), main.next_slice());
    let flag = builder.preprocessed().current_slice()[pre + 1];
    for index in 0..STATE_BITS {
        builder
            .when_transition()
            .when(flag)
            .assert_eq(next[offset + index], local[offset + rho_source(index)]);
    }
}
