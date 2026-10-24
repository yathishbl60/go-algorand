// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACC_INPUT, ACC_OUTPUT, CARRY, MlKemDotAir, PRODUCT, WORK};
use crate::{
    Value,
    mlkem_field::{Add, eval_add},
    mlkem_product_air::{MlKemProductAir, canonical},
    segment_builder::SegmentBuilder,
};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemDotAir {
    pub(super) fn eval_arithmetic<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        MlKemProductAir.eval_arithmetic(&mut SegmentBuilder::new(builder, WORK..WORK + 1195, 0..0));
        let main = builder.main();
        let row = main.current_slice();
        for lane in 0..2 {
            let add = Add {
                left: canonical(row, ACC_INPUT + 25 * lane),
                right: canonical(row, PRODUCT + 25 * lane),
                output: canonical(row, ACC_OUTPUT + 25 * lane),
                carry: row[CARRY + lane],
            };
            eval_add(builder, &add);
        }
    }
}
