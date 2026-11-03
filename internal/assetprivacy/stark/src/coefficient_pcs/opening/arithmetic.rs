// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::E;
use p3_field::PrimeCharacteristicRing;

pub(super) fn dot(alpha: E, values: &[E]) -> E {
    values
        .iter()
        .rev()
        .fold(E::ZERO, |sum, &value| sum * alpha + value)
}

pub(super) fn accumulate(output: &mut [E], compressed: &[E], inverse: &[E], claim: E, scale: E) {
    for ((value, &row), &denominator) in output.iter_mut().zip(compressed).zip(inverse) {
        *value += (claim - scale * row) * denominator;
    }
}
