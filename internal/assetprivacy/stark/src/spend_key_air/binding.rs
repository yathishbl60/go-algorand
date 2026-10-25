// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCOUNT, COMMIT, EPOCH, HASH_WIDTH, MESSAGE_BYTES, NONZERO, OWNER, ROLE, SpendKeyAir, layout,
};
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl SpendKeyAir {
    pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        self.hash.eval(&mut PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..HASH_WIDTH,
            output: COMMIT..COMMIT + 32,
        });
    }
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (byte, expected) in self
            .prefix
            .iter()
            .copied()
            .enumerate()
            .chain(core::iter::once((ROLE, 2)))
        {
            for bit in 0..8 {
                let value = Value::from_bool(expected & (1 << bit) != 0);
                builder.assert_eq(
                    main.current_slice()[NUM_KECCAK_COLS + byte * 8 + bit],
                    value,
                );
            }
        }
    }
    pub(super) fn eval_digest_carry<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for limb in COMMIT..COMMIT + 32 {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[limb], main.next_slice()[limb]);
        }
    }
    pub(super) fn eval_nonzero<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (index, (first, end)) in [
            (ACCOUNT, ROLE),
            (EPOCH, super::CREATED),
            (OWNER, MESSAGE_BYTES),
        ]
        .into_iter()
        .enumerate()
        {
            let sum: AB::Expr = layout::message_bits(main.current_slice(), first, end)
                .iter()
                .copied()
                .map(Into::into)
                .sum();
            builder.assert_one(sum * main.current_slice()[NONZERO + index]);
        }
    }
}
