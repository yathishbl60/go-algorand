// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, EPOCH, EnvelopeAir, NONZERO, RHO, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl EnvelopeAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for column in 0..WORK {
            let current = main.current_slice()[column];
            builder
                .when_transition()
                .assert_eq(current, main.next_slice()[column]);
            if (RHO..COMMIT).contains(&column) || (EPOCH..NONZERO).contains(&column) {
                builder.assert_bool(current);
            }
        }
    }

    pub(super) fn eval_epoch<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let sum: AB::Expr = row[EPOCH..NONZERO].iter().copied().map(Into::into).sum();
        builder.assert_one(sum * row[NONZERO]);
    }

    pub(super) fn eval_context<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for (index, &value) in self.context.bytes().iter().enumerate() {
            let public = builder.public_values()[index];
            builder.assert_eq(public, Value::from_u8(value));
        }
        let main = builder.main();
        for (limb, bytes) in self
            .context
            .commitment()
            .as_chunks::<2>()
            .0
            .iter()
            .enumerate()
        {
            let expected = Value::from_u16(u16::from_le_bytes(*bytes));
            builder.assert_eq(main.current_slice()[COMMIT + limb], expected);
        }
    }
}

pub(super) fn equal<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    left: &[AB::Var],
    right: &[AB::Var],
) {
    assert_eq!(left.len(), right.len(), "fixed envelope binding width");
    for (&left, &right) in left.iter().zip(right) {
        builder.when(flag).assert_eq(left, right);
    }
}
