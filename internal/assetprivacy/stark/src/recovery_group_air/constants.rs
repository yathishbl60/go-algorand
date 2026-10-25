// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{POSITIVE, RecoveryGroupAir, SOURCE_ACCOUNT, SOURCE_OWNER, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl RecoveryGroupAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (column, &value) in row[..WORK].iter().enumerate() {
            builder
                .when_transition()
                .assert_eq(value, main.next_slice()[column]);
        }
        for &bit in &row[SOURCE_ACCOUNT..SOURCE_OWNER] {
            builder.assert_bool(bit);
        }
        for &bit in &row[64..128] {
            builder.assert_zero(bit);
        }
        self.eval_absent(builder);
    }
    fn eval_absent<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (index, present) in [self.operation.private(), self.operation.partial()]
            .into_iter()
            .enumerate()
        {
            if !present {
                for &bit in &row[128 + index * 64..192 + index * 64] {
                    builder.assert_zero(bit);
                }
                builder.assert_zero(row[POSITIVE + index]);
            }
        }
    }
}
