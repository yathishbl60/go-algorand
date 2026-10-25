// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, FLAGS, SpendGroupAir, WORK};
use crate::{
    Value,
    amounts::{self, Amounts},
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl SpendGroupAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        if self.operation.output_count() == 0 {
            for &bit in &main.current_slice()[128..CARRY] {
                builder.assert_zero(bit);
            }
        }
    }
    pub(super) fn eval_amounts<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let values = core::array::from_fn(|index| bits(row, index));
        let carry = row[CARRY..WORK].try_into().expect("carry width");
        amounts::eval(
            builder,
            &Amounts { values, carry },
            self.operation.conversion(),
        );
    }
    pub(super) fn eval_conversion<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let start = self.operation.conversion_start();
        for (index, value) in self.operation.conversion_values().into_iter().enumerate() {
            let public = builder.public_values()[start + index];
            builder.assert_eq(public, Value::from_u16(value));
        }
    }
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..2 {
            let main = builder.main();
            let row = main.current_slice();
            let child = crate::spendable_note_air::wires(
                &row[WORK..WORK + crate::spendable_note_air::WIDTH],
            )
            .expect("spendable note width");
            let root = bits(row, index);
            let flag = builder.preprocessed().current_slice()[FLAGS + index * 4];
            for (&root, &value) in root.iter().zip(child.value_bits) {
                builder.when(flag).assert_eq(root, value);
            }
        }
        for index in 0..self.operation.output_count() {
            self.eval_output_value(builder, index);
        }
    }
    fn eval_output_value<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let value = self
            .output
            .value_bits(&row[WORK..WORK + self.output.width()])
            .expect("output width");
        let flag = builder.preprocessed().current_slice()[FLAGS + (index + 2) * 4];
        for (root, value) in bits(row, index + 2).into_iter().zip(value) {
            builder.when(flag).assert_eq(root, value);
        }
    }
}

fn bits<T: Copy>(row: &[T], index: usize) -> [T; 64] {
    row[index * 64..index * 64 + 64]
        .try_into()
        .expect("amount width")
}
