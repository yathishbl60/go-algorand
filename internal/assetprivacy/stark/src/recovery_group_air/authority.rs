// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::RecoveryGroupAir;
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

impl RecoveryGroupAir {
    pub(super) fn eval_authority<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let start = self.operation.authority_start();
        for (index, byte) in self.authority.into_iter().enumerate() {
            let public = builder.public_values()[start + index];
            builder.assert_eq(public, Value::from_u8(byte));
        }
    }
    pub(super) fn eval_conversion<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let start = self.operation.conversion_start();
        for (index, value) in self.operation.credit_values().into_iter().enumerate() {
            let public = builder.public_values()[start + index];
            builder.assert_eq(public, Value::from_u16(value));
        }
    }
}
