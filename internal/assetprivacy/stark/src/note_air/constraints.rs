// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

use super::{CONTENT_SIZE, ENCODED_SIZE, NONZERO, NoteCommitmentAir, OPENING_START};
use crate::Value;

impl NoteCommitmentAir {
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (i, byte) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                let expected = Value::from_bool(byte & (1 << bit) != 0);
                builder.assert_eq(
                    main.current_slice()[NUM_KECCAK_COLS + i * 8 + bit],
                    expected,
                );
            }
        }
    }

    pub(super) fn eval_padding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for byte in CONTENT_SIZE..ENCODED_SIZE {
            for bit in 0..8 {
                let offset = NUM_KECCAK_COLS + (OPENING_START + byte) * 8 + bit;
                builder.assert_zero(main.current_slice()[offset]);
            }
        }
    }

    pub(super) fn eval_nonzero<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let offset = NUM_KECCAK_COLS + OPENING_START * 8;
        for (i, &(start, end)) in NONZERO.iter().enumerate() {
            let bits = &main.current_slice()[offset + start * 8..offset + end * 8];
            let sum: AB::Expr = bits.iter().map(|&bit| bit.into()).sum();
            builder.assert_one(sum * main.current_slice()[self.hash.width() + i]);
        }
    }
}
