// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

use super::Sha3Air;
use crate::Value;

impl Sha3Air {
    pub(super) fn eval_message<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for i in NUM_KECCAK_COLS..self.block_offset() {
            let local = main.current_slice()[i];
            builder.assert_bool(local);
            builder
                .when_transition()
                .assert_eq(local, main.next_slice()[i]);
        }
    }

    pub(super) fn message_bit<AB: AirBuilder<F = Value>>(
        &self,
        row: &[AB::Var],
        bit: usize,
    ) -> AB::Expr {
        if bit < self.message_len * 8 {
            return row[NUM_KECCAK_COLS + bit].into();
        }
        let byte = bit / 8;
        let suffix = self.domain_suffix * u8::from(byte == self.message_len);
        let last = if byte + 1 == self.blocks() * self.rate_bytes() {
            0x80
        } else {
            0
        };
        AB::Expr::from_bool((suffix | last) & (1 << (bit % 8)) != 0)
    }
}
