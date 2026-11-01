// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, PARITY_BITS, values};
use crate::serial_keccak::{parity_index, state_index};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField64};

impl<F: PrimeField64> CompactKeccakAir<F> {
    pub(super) fn eval_parity<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for index in 0..PARITY_BITS {
            let (x, z) = (index / 64, index % 64);
            let mut after: AB::Expr = row[state_index(x, 0, z)].into();
            for y in 1..5 {
                after = after.xor(&row[state_index(x, y, z)].into());
            }
            let before = values::xor3::<AB::Expr, _>([
                row[parity_index(x, z)],
                row[parity_index((x + 4) % 5, z)],
                row[parity_index((x + 1) % 5, (z + 63) % 64)],
            ]);
            builder.assert_eq(after, before);
        }
    }
}
