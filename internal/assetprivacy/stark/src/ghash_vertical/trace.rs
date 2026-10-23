// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCUMULATOR, BITS, GhashProductAir, INPUT, LEFT, OUTPUT, POWER, RIGHT, ROWS, SELECTED, WIDTH,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl GhashProductAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * BITS);
        for round in 0..BITS {
            values[round * BITS + round] = Value::ONE;
        }
        RowMajorMatrix::new(values, BITS)
    }

    /// Generates a private product trace, not a proof or an encryption authorization.
    pub fn trace(&self, left: [u8; 16], right: [u8; 16]) -> RowMajorMatrix<Value> {
        let (left, right) = (u128::from_be_bytes(left), u128::from_be_bytes(right));
        let output = product(left, right);
        let mut state = State {
            power: right,
            sum: 0,
        };
        let mut values = Value::zero_vec(ROWS * WIDTH);
        for (round, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            write_row(row, [left, right, output], &mut state, round);
        }
        RowMajorMatrix::new(values, WIDTH)
    }
}

struct State {
    power: u128,
    sum: u128,
}

impl State {
    fn advance(&mut self, input: bool) {
        self.sum ^= self.power & 0u128.wrapping_sub(u128::from(input));
        self.power = (self.power >> 1) ^ ((0xe1u128 << 120) & 0u128.wrapping_sub(self.power & 1));
    }
}

fn product(left: u128, right: u128) -> u128 {
    let mut state = State {
        power: right,
        sum: 0,
    };
    for round in 0..BITS {
        state.advance(left & (1 << (127 - round)) != 0);
    }
    state.sum
}

fn write_row(row: &mut [Value], fixed: [u128; 3], state: &mut State, round: usize) {
    let input = round < BITS && fixed[0] & (1 << (127 - round)) != 0;
    for (start, value) in [LEFT, RIGHT, OUTPUT].into_iter().zip(fixed) {
        write_bits(row, start, value);
    }
    write_bits(row, POWER, state.power);
    write_bits(row, ACCUMULATOR, state.sum);
    write_bits(
        row,
        SELECTED,
        state.power & 0u128.wrapping_sub(u128::from(input)),
    );
    row[INPUT] = Value::from_bool(input);
    if round < BITS {
        state.advance(input);
    }
}

fn write_bits(row: &mut [Value], start: usize, value: u128) {
    for bit in 0..BITS {
        row[start + bit] = Value::from_bool(value & (1 << (127 - bit)) != 0);
    }
}
