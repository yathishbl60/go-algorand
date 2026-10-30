// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, RHO, ROWS, SIGMA, WORK};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn write_globals(
    values: &mut [Value],
    target: (usize, usize, usize),
    seed: [u8; 32],
    expanded: [u8; 64],
    coefficients: [[u16; 256]; 6],
) {
    for parent in values.chunks_exact_mut(target.0).skip(target.2).take(ROWS) {
        let row = &mut parent[target.1..target.1 + WORK];
        write_bits(&mut row[..RHO], &seed);
        write_bits(&mut row[RHO..SIGMA], &expanded[..32]);
        write_bits(&mut row[SIGMA..COEFFICIENTS], &expanded[32..]);
        for (cell, value) in row[COEFFICIENTS..WORK]
            .iter_mut()
            .zip(coefficients.iter().flatten())
        {
            *cell = Value::from_u16(*value);
        }
    }
}

fn write_bits(target: &mut [Value], bytes: &[u8]) {
    for (bit, cell) in target.iter_mut().enumerate() {
        *cell = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
