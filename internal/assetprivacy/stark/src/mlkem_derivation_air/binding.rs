// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemDerivationAir;
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemDerivationAir {
    pub(super) fn eval_hash_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for limb in 0..16 {
            let start = self.expand_message() + 256 + limb * 16;
            let decoded = (0..16)
                .rev()
                .fold(AB::Expr::ZERO, |v, bit| v.double() + row[start + bit]);
            builder.assert_eq(row[self.digest_start() + limb], decoded);
        }
        self.eval_constants(builder);
    }

    fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (column, _) in row
            .iter()
            .enumerate()
            .take(self.scratch())
            .skip(self.digest_start())
        {
            builder
                .when_transition()
                .assert_eq(row[column], main.next_slice()[column]);
        }
    }

    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[self.output_start()..self.scratch()] {
            builder.assert_bool(bit);
        }
        for limb in 0..32 {
            let start = self.output_start() + limb * 16;
            let decoded = (0..16)
                .rev()
                .fold(AB::Expr::ZERO, |v, bit| v.double() + row[start + bit]);
            builder.assert_eq(row[self.digest_start() + 16 + limb], decoded);
        }
    }
}
