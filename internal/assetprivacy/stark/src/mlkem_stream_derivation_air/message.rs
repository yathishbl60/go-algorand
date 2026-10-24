// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HASH, MESSAGE, WINDOW};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub(super) fn key_bit<AB: AirBuilder<F = Value>>(
    row: &[AB::Var],
    block: usize,
    bit: usize,
) -> AB::Expr {
    let global = block * 1088 + bit;
    if global < 9472 {
        return row[WINDOW + block * 1088 % 12 + bit].into();
    }
    AB::Expr::from_bool(matches!(global, 9473 | 9474 | 9791))
}

pub(super) fn expand_bit<AB: AirBuilder<F = Value>>(row: &[AB::Var], bit: usize) -> AB::Expr {
    match bit {
        0..256 => row[MESSAGE + bit].into(),
        256..512 => row[HASH + bit - 256].into(),
        512 => AB::Expr::ZERO,
        513 | 514 | 575 => AB::Expr::ONE,
        _ => AB::Expr::ZERO,
    }
}

pub(super) fn bits<AB: AirBuilder<F = Value>>(bits: &[AB::Var]) -> AB::Expr {
    bits.iter()
        .rev()
        .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit)
}
