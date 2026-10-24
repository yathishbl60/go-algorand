// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCEPT, BUFFER, CANDIDATE, CURSOR, GAP, MlKemStreamMatrixAir, OUTPUT, RATE, REJECT, SEED,
    trace::Prepared,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl MlKemStreamMatrixAir {
    pub(super) fn write_row(
        &self,
        row: &mut [Value],
        input: &Prepared,
        step: (usize, usize, usize),
    ) -> usize {
        let (block, offset, cursor) = step;
        write_fixed(row, input, block, offset);
        write_candidate(row, &input.prefix[block * 168..][..168], offset, cursor)
    }
}

fn write_fixed(row: &mut [Value], input: &Prepared, block: usize, offset: usize) {
    write_bits(&mut row[SEED..BUFFER], &input.seed);
    let state = &input.states[block + usize::from(offset >= 24)];
    for limb in 0..100 {
        let value = if block == 0 && offset < 24 {
            0
        } else {
            (state[limb / 4] >> (16 * (limb % 4))) as u16
        };
        row[BUFFER + limb] = Value::from_u16(value);
    }
    write_bits(&mut row[RATE..OUTPUT], &input.prefix[block * 168..][..168]);
    for (target, &value) in row[OUTPUT..CURSOR].iter_mut().zip(&input.coefficients) {
        *target = Value::from_u16(value);
    }
}

fn write_candidate(row: &mut [Value], prefix: &[u8], offset: usize, cursor: usize) -> usize {
    row[CURSOR + cursor] = Value::ONE;
    let active = (24..136).contains(&offset);
    let candidate = if active {
        candidate(prefix, offset - 24)
    } else {
        0
    };
    write_u12(&mut row[CANDIDATE..GAP], candidate);
    write_u12(&mut row[GAP..REJECT], (candidate + 767) & 4095);
    row[REJECT] = Value::from_bool(candidate >= 3329);
    let accept = active && candidate < 3329 && cursor < 256;
    row[ACCEPT] = Value::from_bool(accept);
    cursor + usize::from(accept)
}

fn write_bits(row: &mut [Value], input: &[u8]) {
    for (index, value) in row.iter_mut().enumerate() {
        *value = Value::from_bool(input[index / 8] & (1 << (index % 8)) != 0);
    }
}
fn write_u12(row: &mut [Value], input: u16) {
    for (index, value) in row.iter_mut().enumerate() {
        *value = Value::from_bool(input & (1 << index) != 0);
    }
}
fn candidate(prefix: &[u8], index: usize) -> u16 {
    let bytes = &prefix[index / 2 * 3..][..3];
    if index.is_multiple_of(2) {
        u16::from(bytes[0]) + (u16::from(bytes[1] & 15) << 8)
    } else {
        u16::from(bytes[1] >> 4) + (u16::from(bytes[2]) << 4)
    }
}
