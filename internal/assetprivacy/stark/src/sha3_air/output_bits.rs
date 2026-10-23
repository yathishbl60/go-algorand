// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::{KeccakCols, RC};

use crate::Value;

// Valid only under the final-round selector; KeccakAir constrains all source bits.
pub(super) fn final_bit<AB: AirBuilder<F = Value>>(
    row: &KeccakCols<AB::Var>,
    bit: usize,
) -> AB::Expr {
    let (x, y, z) = ((bit / 64) % 5, bit / 320, bit % 64);
    let a: AB::Expr = row.b(x, y, z).into();
    let b: AB::Expr = row.b((x + 1) % 5, y, z).into();
    let c: AB::Expr = row.b((x + 2) % 5, y, z).into();
    let chi = a.xor(&b.andn(&c));
    if x == 0 && y == 0 && RC[23] & (1 << z) != 0 {
        AB::Expr::ONE - chi
    } else {
        chi
    }
}
