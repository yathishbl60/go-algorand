// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PARITY_BITS, parity_index, state_index};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

// Ungated XOR5 has degree five. Boolean state implies Boolean column parity.
pub(super) fn eval<AB: AirBuilder<F = Value>>(builder: &mut AB, offset: usize) {
    let main = builder.main();
    let local = main.current_slice();
    for index in 0..PARITY_BITS {
        let (x, z) = (index / 64, index % 64);
        let mut expected: AB::Expr = local[offset + state_index(x, 0, z)].into();
        for y in 1..5 {
            expected = expected.xor(&local[offset + state_index(x, y, z)].into());
        }
        builder.assert_eq(local[offset + parity_index(x, z)], expected);
    }
}
