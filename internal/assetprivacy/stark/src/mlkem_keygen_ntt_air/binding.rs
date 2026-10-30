// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE, FLAGS, MlKemKeygenNttAir, NTT_WIDTH, RAW, TRANSFORMED, WIDTH, WORK};
use crate::{Value, mlkem_ntt_air};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemKeygenNttAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for range in [0..CORE, TRANSFORMED..WIDTH] {
            for (offset, &value) in row[range.clone()].iter().enumerate() {
                builder
                    .when_transition()
                    .assert_eq(value, main.next_slice()[range.start + offset]);
            }
        }
    }
    pub(super) fn eval_transforms<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for i in 0..6 {
            self.eval_transform(builder, i);
        }
    }
    fn eval_transform<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, i: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let child = mlkem_ntt_air::wires(&row[WORK..WORK + NTT_WIDTH]).expect("NTT width");
        let active = builder.preprocessed().current_slice()[FLAGS + 4 * (i + 1)];
        for (j, (&input, &output)) in child.input.iter().zip(child.output).enumerate() {
            builder
                .when(active)
                .assert_eq(row[RAW + i * 256 + j], input);
            builder
                .when(active)
                .assert_eq(row[TRANSFORMED + i * 256 + j], output);
        }
    }
}
