// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CONVERSION_START, TransferAir};
use crate::{
    Value,
    amounts::{self, Amounts},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl TransferAir {
    pub(super) fn eval_conservation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let values = core::array::from_fn(|i| self.value_bits(row, i));
        let carry = row[self.carry_start()..self.width()]
            .try_into()
            .expect("carry width");
        amounts::eval(builder, &Amounts { values, carry }, self.conversion);
    }

    fn value_bits<T: Copy>(&self, row: &[T], index: usize) -> [T; 64] {
        if index < 2 {
            let start = index * self.input.width();
            return self
                .input
                .value_bits(&row[start..start + self.input.width()])
                .expect("input width");
        }
        let start = 2 * self.input.width() + (index - 2) * self.output.width();
        self.output
            .value_bits(&row[start..start + self.output.width()])
            .expect("output width")
    }

    pub(super) fn eval_conversion<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for (index, amount) in [self.conversion.debit, self.conversion.credit]
            .into_iter()
            .enumerate()
        {
            for limb in 0..4 {
                let expected = Value::from_u16((amount >> (limb * 16)) as u16);
                let public = builder.public_values()[CONVERSION_START + index * 4 + limb];
                builder.assert_eq(public, expected);
            }
        }
    }
}
