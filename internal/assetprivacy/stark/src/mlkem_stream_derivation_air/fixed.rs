// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{KECCAK, MlKemStreamDerivationAir, RHO, Value};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemStreamDerivationAir {
    pub(super) fn eval_fixed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[RHO..KECCAK] {
            builder.assert_bool(bit);
        }
        for (column, &current) in row[..KECCAK].iter().enumerate() {
            builder
                .when_transition()
                .assert_eq(current, main.next_slice()[column]);
        }
    }
}
