// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{STATE_BITS, index::coordinates, parity_index};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

pub(super) fn eval<AB: AirBuilder<F = Value>>(builder: &mut AB, offset: usize, pre: usize) {
    let main = builder.main();
    let (local, next) = (main.current_slice(), main.next_slice());
    let flag = builder.preprocessed().current_slice()[pre];
    for index in 0..STATE_BITS {
        let (x, _, z) = coordinates(index);
        let value: AB::Expr = local[offset + index].into();
        let expected = value.xor3(
            &local[offset + parity_index((x + 4) % 5, z)].into(),
            &local[offset + parity_index((x + 1) % 5, (z + 63) % 64)].into(),
        );
        builder
            .when_transition()
            .when(flag)
            .assert_eq(next[offset + index], expected);
    }
}
