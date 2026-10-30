// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HASH_WIDTH, OwnerRegistrationAir, PREFIX, WORK};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl OwnerRegistrationAir {
    pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.hash.eval(&mut FrameBuilder::new(
            builder,
            Frame {
                columns: WORK..WORK + HASH_WIDTH,
                preprocessing: 0..0,
                public: 0..32,
                flags: 0,
            },
        ));
    }
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for (index, byte) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                builder.assert_eq(
                    main.current_slice()[WORK + NUM_KECCAK_COLS + index * 8 + bit],
                    Value::from_bool(byte & (1 << bit) != 0),
                );
            }
        }
    }
    pub(super) fn eval_secret<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let start = WORK + NUM_KECCAK_COLS + PREFIX * 8;
        let sum: AB::Expr = main.current_slice()[start..start + 256]
            .iter()
            .map(|&value| value.into())
            .sum();
        builder.assert_one(sum * main.current_slice()[0]);
    }
    pub(super) fn eval_group<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            builder
                .when_transition()
                .assert_eq(main.current_slice()[column], main.next_slice()[column]);
        }
        for limb in 0..32 {
            builder.assert_eq(
                main.current_slice()[1 + limb],
                builder.public_values()[32 + limb],
            );
        }
    }
}
