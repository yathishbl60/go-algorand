// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, GH_FLAGS, GH_START, GcmVerticalAir, H, MASK, PT, WIDTH};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl GcmVerticalAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for range in [0..1920, PT..WIDTH] {
            for column in range {
                let bit = main.current_slice()[column];
                builder.assert_bool(bit);
                builder
                    .when_transition()
                    .assert_eq(bit, main.next_slice()[column]);
            }
        }
    }

    pub(super) fn eval_hash_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[GH_FLAGS];
        for (local, global, width) in [(0, H, 128), (128, AAD, 512)] {
            for bit in 0..width {
                let source = global + bit / 8 * 8 + 7 - bit % 8;
                builder
                    .when(active)
                    .assert_eq(row[GH_START + local + bit], row[source]);
            }
        }
    }

    pub(super) fn eval_tag<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[GH_FLAGS];
        for limb in 0..8 {
            let tag = (0..16).rev().fold(AB::Expr::ZERO, |value, bit| {
                let wire = limb * 16 + bit;
                let hash = row[GH_START + 640 + wire / 8 * 8 + 7 - wire % 8];
                value.double() + AB::Expr::from(hash).xor(&row[MASK + wire].into())
            });
            let public = builder.public_values()[268 + limb];
            builder.when(active).assert_eq(tag, public);
        }
    }
}
