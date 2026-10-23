// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

use super::OwnedNoteAir;
use crate::Value;

impl OwnedNoteAir {
    pub(super) fn eval_owner_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let offset = self.note.width() + NUM_KECCAK_COLS;
        for (i, byte) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                builder.assert_eq(
                    main.current_slice()[offset + i * 8 + bit],
                    Value::from_bool(byte & (1 << bit) != 0),
                );
            }
        }
    }

    pub(super) fn eval_secret<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let start = self.note.width() + NUM_KECCAK_COLS + self.prefix.len() * 8;
        let sum: AB::Expr = main.current_slice()[start..start + 256]
            .iter()
            .map(|&v| v.into())
            .sum();
        builder.assert_one(sum * main.current_slice()[self.digest_offset() + 64]);
    }

    pub(super) fn eval_owner_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for limb in 0..32 {
            let expected = (0..16).rev().fold(AB::Expr::ZERO, |v, bit| {
                let column = self
                    .note
                    .opening_bit_column(73 + limb * 2 + bit / 8, bit % 8);
                v.double() + main.current_slice()[column]
            });
            builder.assert_eq(
                main.current_slice()[self.digest_offset() + 32 + limb],
                expected,
            );
        }
    }

    pub(super) fn eval_commitment<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for i in self.digest_offset()..self.digest_offset() + 32 {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[i], main.next_slice()[i]);
        }
    }
}
