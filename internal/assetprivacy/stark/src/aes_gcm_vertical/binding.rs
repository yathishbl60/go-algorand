// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{AesGcmAir, H, INPUT, KEY, MASK, NONCE, PT};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl AesGcmAir {
    pub(super) fn eval_bindings<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for &value in main.current_slice() {
            builder.assert_bool(value);
        }
        for (start, width) in [(KEY, 1920), (PT, 4096), (H, 128), (MASK, 128)] {
            for bit in 0..width {
                builder.when_transition().assert_eq(
                    main.current_slice()[start + bit],
                    main.next_slice()[start + bit],
                );
            }
        }
        self.eval_nonce(builder);
        for block in 0..34 {
            self.eval_input(builder, block);
        }
    }

    fn eval_nonce<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for byte in 0..12 {
            let value = (0..8).rev().fold(AB::Expr::ZERO, |v, bit| {
                v.double() + main.current_slice()[NONCE + byte * 8 + bit]
            });
            let public = builder.public_values()[byte];
            builder.assert_eq(value, public);
        }
    }

    fn eval_input<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, block: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let flag = builder.preprocessed().current_slice()[27 + block];
        let counter = (block as u32).to_be_bytes();
        for byte in 0..16 {
            for bit in 0..8 {
                let expected = if block == 0 {
                    AB::Expr::ZERO
                } else if byte < 12 {
                    row[NONCE + byte * 8 + bit].into()
                } else {
                    AB::Expr::from_bool(counter[byte - 12] & (1 << bit) != 0)
                };
                builder
                    .when(flag)
                    .assert_eq(row[INPUT + byte * 8 + bit], expected);
            }
        }
    }
}
