// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE, OwnerRegistrationAir, WORK};
use crate::{Value, serial_keccak::STATE_BITS};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

const SECRET_BITS: core::ops::Range<usize> = 40..296;

impl OwnerRegistrationAir {
    pub(super) fn eval_absorption<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let absorb = builder.preprocessed().current_slice()[3];
        for bit in 0..STATE_BITS {
            if SECRET_BITS.contains(&bit) {
                continue;
            }
            let local: AB::Expr = main.current_slice()[WORK + bit].into();
            let expected = if self.fixed_bit(bit) {
                AB::Expr::ONE - local
            } else {
                local
            };
            builder
                .when_transition()
                .when(absorb)
                .assert_eq(main.next_slice()[WORK + bit], expected);
        }
    }

    pub(super) fn eval_secret<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let absorb = builder.preprocessed().current_slice()[3];
        let sum: AB::Expr = SECRET_BITS
            .map(|bit| {
                let local: AB::Expr = main.current_slice()[WORK + bit].into();
                local.xor(&main.next_slice()[WORK + bit].into())
            })
            .sum();
        builder
            .when_transition()
            .when(absorb)
            .assert_one(sum * main.current_slice()[0]);
    }

    fn fixed_bit(&self, bit: usize) -> bool {
        if bit >= 72 * 8 {
            return false;
        }
        let index = 72 + bit / 8;
        let byte = self.prefix.get(index).copied().unwrap_or(0)
            | (6 * u8::from(index == MESSAGE))
            | (0x80 * u8::from(index == 143));
        byte & (1 << (bit % 8)) != 0
    }
}
