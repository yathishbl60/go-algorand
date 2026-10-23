// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCUMULATOR, BITS, GhashProductAir, POWER, SELECTED};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl GhashProductAir {
    pub(super) fn eval_steps<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let active = active::<AB>(builder);
        for bit in 0..BITS {
            let sum = AB::Expr::from(row[ACCUMULATOR + bit]).xor(&row[SELECTED + bit].into());
            builder
                .when_transition()
                .assert_eq(main.next_slice()[ACCUMULATOR + bit], sum);
            let prior: AB::Expr = row[POWER + bit].into();
            let next = prior.clone() + active.clone() * (shifted::<AB>(row, bit) - prior);
            builder
                .when_transition()
                .assert_eq(main.next_slice()[POWER + bit], next);
        }
    }
}

fn active<AB: AirBuilder<F = Value>>(builder: &AB) -> AB::Expr {
    builder
        .preprocessed()
        .current_slice()
        .iter()
        .map(|&flag| flag.into())
        .sum()
}

fn shifted<AB: AirBuilder<F = Value>>(row: &[AB::Var], bit: usize) -> AB::Expr {
    let shift = if bit == 0 {
        AB::Expr::ZERO
    } else {
        row[POWER + bit - 1].into()
    };
    if matches!(bit, 0 | 1 | 2 | 7) {
        shift.xor(&row[POWER + BITS - 1].into())
    } else {
        shift
    }
}
