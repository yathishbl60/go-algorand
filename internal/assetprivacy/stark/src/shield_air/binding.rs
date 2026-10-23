// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEBIT_START, ShieldAir};
use crate::{
    Value,
    amounts::{self, Amounts, PublicValue},
    public_segment_builder::SegmentBuilder,
};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl ShieldAir {
    pub(super) fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let window = builder.main();
        let start = index * self.output.width();
        self.output.eval(&mut SegmentBuilder {
            inner: builder,
            window,
            columns: start..start + self.output.width(),
            public: index * 32..index * 32 + 32,
        });
    }

    pub(super) fn eval_conservation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let zero = row[self.auxiliary_start()];
        builder.assert_zero(zero);
        let carry = row[self.auxiliary_start() + 1..self.width()]
            .try_into()
            .expect("carry width");
        let values = self.amount_bits(row);
        amounts::eval(
            builder,
            &Amounts { values, carry },
            PublicValue {
                debit: self.debit,
                credit: 0,
            },
        );
    }

    fn amount_bits<T: Copy>(&self, row: &[T]) -> [[T; 64]; 4] {
        let width = self.output.width();
        let a = self.output.value_bits(&row[..width]).expect("output width");
        let b = self
            .output
            .value_bits(&row[width..2 * width])
            .expect("output width");
        let zero = row[self.auxiliary_start()];
        [[zero; 64], [zero; 64], a, b]
    }

    pub(super) fn eval_debit<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for limb in 0..4 {
            let expected = Value::from_u16((self.debit >> (limb * 16)) as u16);
            let public = builder.public_values()[DEBIT_START + limb];
            builder.assert_eq(public, expected);
        }
    }
}
