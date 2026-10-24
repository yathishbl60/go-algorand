// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    DOT, FWD_SLOT, GLOBAL_END, INV_SLOT, MESSAGE, MlKemPkeAir, NOISE, RAW_CARRY, RHAT, WORK,
};
use crate::{Value, mlkem_ntt_air};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemPkeAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for range in [0..WORK, NOISE..GLOBAL_END] {
            for column in range {
                builder
                    .when_transition()
                    .assert_eq(row[column], main.next_slice()[column]);
            }
        }
        for &bit in &row[MESSAGE..MESSAGE + 256] {
            builder.assert_bool(bit);
        }
    }

    pub(super) fn eval_ntt_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for slot in 0..3 {
            self.bind_ntt(builder, slot, false);
        }
        for slot in 0..4 {
            self.bind_ntt(builder, slot, true);
        }
    }

    fn bind_ntt<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, slot: usize, inverse: bool) {
        let main = builder.main();
        let row = main.current_slice();
        let wires = mlkem_ntt_air::wires(&row[WORK..WORK + 920]).expect("fixed NTT columns");
        let column = if inverse { INV_SLOT } else { FWD_SLOT };
        let flag = builder.preprocessed().current_slice()[column + slot];
        let input = if inverse { DOT } else { NOISE };
        for index in 0..256 {
            builder
                .when(flag)
                .assert_eq(wires.input[index], row[input + slot * 256 + index]);
            if !inverse {
                builder
                    .when(flag)
                    .assert_eq(wires.output[index], row[RHAT + slot * 256 + index]);
            }
        }
    }

    pub(super) fn eval_raw_carry<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let current =
            mlkem_ntt_air::wires(&main.current_slice()[WORK..WORK + 920]).expect("NTT columns");
        let next = mlkem_ntt_air::wires(&main.next_slice()[WORK..WORK + 920]).expect("NTT columns");
        let flag = builder.preprocessed().current_slice()[RAW_CARRY];
        for index in 0..256 {
            builder
                .when(flag)
                .assert_eq(current.output[index], next.output[index]);
        }
    }
}
