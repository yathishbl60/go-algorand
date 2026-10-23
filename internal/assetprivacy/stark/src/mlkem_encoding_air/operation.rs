// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LOCAL_BITS, MlKemEncodingAir, ROLLOVER, view};
use crate::{
    Value,
    mlkem_field::{eval_canonical, eval_compression},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemEncodingAir {
    pub(super) fn eval_operation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = &main.current_slice()[self.scratch()..];
        match self.bits {
            4 => eval_compression(builder, &view::compression::<_, 4>(row)),
            10 => eval_compression(builder, &view::compression::<_, 10>(row)),
            _ => self.eval_uncompressed(builder),
        }
    }

    fn eval_uncompressed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = &main.current_slice()[self.scratch()..];
        let input = view::canonical(row, 0);
        eval_canonical(builder, &input);
        if self.bits == 12 {
            for bit in 0..12 {
                builder.assert_eq(row[LOCAL_BITS + bit], input.bits[bit]);
            }
        } else {
            builder.assert_eq(input.value, row[LOCAL_BITS] * Value::from_u16(1665));
        }
        for &unused in &row[ROLLOVER..] {
            builder.assert_zero(unused);
        }
    }
}
