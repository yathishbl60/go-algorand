// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::witness_state::State;
use super::{AAD, ACTIVE_ROWS, BITS, DATA, H, INPUT, RESULT, SELECTED, V, X, Y, Z};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn fixed(row: &mut [Value], h: u128, associated: &[u8; 64], result: u128) {
    bits(row, H, h);
    bits(row, RESULT, result);
    for (byte, value) in associated.iter().enumerate() {
        for bit in 0..8 {
            row[AAD + byte * 8 + bit] = Value::from_bool(value & (1 << (7 - bit)) != 0);
        }
    }
}

pub(super) fn state(row: &mut [Value], state: &State, round: usize) {
    let input = round < ACTIVE_ROWS && state.input(round % BITS);
    let selected = state.v & 0u128.wrapping_sub(u128::from(input));
    for (offset, value) in [
        (Y, state.y),
        (X, state.x),
        (V, state.v),
        (Z, state.z),
        (DATA, state.data),
        (SELECTED, selected),
    ] {
        bits(row, offset, value);
    }
    row[INPUT] = Value::from_bool(input);
}

fn bits(row: &mut [Value], offset: usize, value: u128) {
    for bit in 0..BITS {
        row[offset + bit] = Value::from_bool(value & (1 << (127 - bit)) != 0);
    }
}
