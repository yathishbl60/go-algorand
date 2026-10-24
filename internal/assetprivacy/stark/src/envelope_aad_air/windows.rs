// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CT_BLOCK, EnvelopeAadAir, KEY_BLOCK, RHO, WINDOW, message};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl EnvelopeAadAir {
    pub(super) fn eval_key_windows<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for &bit in &main.current_slice()[WINDOW..WINDOW + 588] {
            builder.assert_bool(bit);
        }
        for block in 3..20 {
            self.eval_key_window(builder, block);
        }
    }
    fn eval_key_window<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[KEY_BLOCK + block - 3];
        let first = message::key_start(block) / 12;
        for coefficient in 0..49 {
            if first + coefficient < 768 {
                let start = WINDOW + 12 * coefficient;
                let value = message::bits::<AB>(&row[start..start + 12]);
                builder
                    .when(flag)
                    .assert_eq(value, row[first + coefficient]);
            }
        }
        self.eval_key_tail(builder, block);
    }
    fn eval_key_tail<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[KEY_BLOCK + block - 3];
        let first = message::key_start(block);
        for bit in 9216usize.saturating_sub(first)..588 {
            let global = first + bit;
            let value = if global < 9472 {
                row[RHO + global - 9216].into()
            } else {
                AB::Expr::ZERO
            };
            builder.when(flag).assert_eq(row[WINDOW + bit], value);
        }
    }
    pub(super) fn eval_ciphertext_windows<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for block in 20..36 {
            self.eval_ciphertext_window(builder, block);
        }
    }
    fn eval_ciphertext_window<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[CT_BLOCK + block - 20];
        let count = if block == 35 { 4 } else { 36 };
        for limb in 0..count {
            let value = message::bits::<AB>(&row[WINDOW + 16 * limb..WINDOW + 16 * (limb + 1)]);
            let public = builder.public_values()[(block - 20) * 36 + limb];
            builder.when(flag).assert_eq(value, public);
        }
        for &bit in &row[WINDOW + count * 16..WINDOW + 588] {
            builder.when(flag).assert_zero(bit);
        }
    }
}
