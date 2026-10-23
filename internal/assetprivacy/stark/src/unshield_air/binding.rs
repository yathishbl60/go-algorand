// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CREDIT_START, UnshieldAir};
use crate::{
    Value,
    amounts::{self, Amounts, PublicValue},
    public_segment_builder::SegmentBuilder,
};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl UnshieldAir {
    pub(super) fn eval_input<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let window = builder.main();
        let start = index * self.input.width();
        self.input.eval(&mut SegmentBuilder {
            inner: builder,
            window,
            columns: start..start + self.input.width(),
            public: index * 64..index * 64 + 64,
        });
    }

    pub(super) fn eval_conservation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        builder.assert_zero(row[self.auxiliary_start()]);
        let carry = row[self.auxiliary_start() + 1..self.width()]
            .try_into()
            .expect("carry width");
        let amounts = Amounts {
            values: self.amount_bits(row),
            carry,
        };
        amounts::eval(
            builder,
            &amounts,
            PublicValue {
                debit: 0,
                credit: self.credit,
            },
        );
    }

    fn amount_bits<T: Copy>(&self, row: &[T]) -> [[T; 64]; 4] {
        let width = self.input.width();
        let a = self.input.value_bits(&row[..width]).expect("input width");
        let b = self
            .input
            .value_bits(&row[width..2 * width])
            .expect("input width");
        let zero = row[self.auxiliary_start()];
        [a, b, [zero; 64], [zero; 64]]
    }

    pub(super) fn eval_credit<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for limb in 0..4 {
            let expected = Value::from_u16((self.credit >> (limb * 16)) as u16);
            let public = builder.public_values()[CREDIT_START + limb];
            builder.assert_eq(public, expected);
        }
    }
}
