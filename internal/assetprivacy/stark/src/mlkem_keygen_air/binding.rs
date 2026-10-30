// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, FLAGS, GROUP, MlKemKeygenAir, TRANSFORMED, WIDTH};
use crate::{Value, mlkem_field::Q};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemKeygenAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (i, &value) in main.current_slice()[TRANSFORMED..WIDTH].iter().enumerate() {
            builder
                .when_transition()
                .assert_eq(value, main.next_slice()[TRANSFORMED + i]);
        }
    }
    pub(super) fn eval_rho<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let active = builder.preprocessed().current_slice()[FLAGS];
        for bit in 0..256 {
            builder.when(active).assert_eq(
                main.current_slice()[256 + bit],
                Value::from_bool(self.key.rho[bit / 8] & (1 << (bit % 8)) != 0),
            );
        }
    }
    pub(super) fn eval_dot_inputs<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[FLAGS + 4];
        for (i, &value) in row[..2304].iter().enumerate() {
            let known = self.key.matrix[(i / 256) % 3][i / (3 * 256)][i % 256];
            builder
                .when(active)
                .assert_eq(value, Value::from_u16(known));
        }
        for &value in &row[2304..3072] {
            builder.when(active).assert_zero(value);
        }
        for (i, &value) in row[3072..3840].iter().enumerate() {
            builder.when(active).assert_eq(value, row[TRANSFORMED + i]);
        }
    }
    pub(super) fn eval_sum<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = builder.preprocessed().current_slice()[FLAGS + 4];
        for i in 0..768 {
            let carry = row[CARRY + i];
            builder.when(active).assert_bool(carry);
            let sum = row[3840 + i] + row[TRANSFORMED + 768 + i];
            let known = Value::from_u16(self.key.polynomials[i / 256][i % 256]);
            builder
                .when(active)
                .assert_eq(sum, carry * Value::from_u16(Q) + known);
        }
    }
    pub(super) fn eval_public<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for (i, &byte) in self.key.encoded.iter().enumerate() {
            builder.assert_eq(builder.public_values()[i], Value::from_u8(byte));
        }
        let main = builder.main();
        for i in 0..32 {
            builder.assert_eq(
                main.current_slice()[GROUP + i],
                builder.public_values()[1184 + i],
            );
        }
    }
}
