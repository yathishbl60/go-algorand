// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, RC_POSITIONS, STATE_BITS, index::coordinates, state_index};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

pub(super) fn eval<AB: AirBuilder<F = Value>>(builder: &mut AB, offset: usize, pre: usize) {
    let main = builder.main();
    let (local, next) = (main.current_slice(), main.next_slice());
    let flags: [AB::Var; PREPROCESSED] = builder.preprocessed().current_slice()
        [pre..pre + PREPROCESSED]
        .try_into()
        .expect("trusted serial Keccak selectors");
    for index in 0..STATE_BITS {
        let chi = chi_bit::<AB>(local, offset, index);
        let flag: AB::Expr = flags[2].into();
        let rc = round_bit::<AB>(&flags, index);
        let constraint = flag * (next[offset + index].into() - chi.clone())
            - rc * (AB::Expr::ONE - chi.double());
        builder.when_transition().assert_zero(constraint);
    }
}

fn chi_bit<AB: AirBuilder<F = Value>>(local: &[AB::Var], offset: usize, index: usize) -> AB::Expr {
    let (x, y, z) = coordinates(index);
    let a: AB::Expr = local[offset + index].into();
    let b: AB::Expr = local[offset + state_index((x + 1) % 5, y, z)].into();
    let c: AB::Expr = local[offset + state_index((x + 2) % 5, y, z)].into();
    a.xor(&((AB::Expr::ONE - b) * c))
}

fn round_bit<AB: AirBuilder<F = Value>>(flags: &[AB::Var; PREPROCESSED], index: usize) -> AB::Expr {
    if index >= 64 {
        return AB::Expr::ZERO;
    }
    match RC_POSITIONS.iter().position(|&bit| bit == index) {
        Some(column) => flags[4 + column].into(),
        None => AB::Expr::ZERO,
    }
}
