// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACC_INPUT, ACC_OUTPUT, ACTIVE, CARRY, DotInputs, STATE, WORK};
use crate::{
    Value,
    mlkem_field::{Q, ntt_gamma},
    mlkem_product_air::{Pair, write_canonical, write_pair},
};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(row: &mut [Value], input: &DotInputs, state: &mut [u16; 256], step: usize) {
    for (index, value) in state.iter().copied().enumerate() {
        row[STATE + index] = Value::from_u16(value);
    }
    if step >= ACTIVE {
        write_pair(&mut row[WORK..WORK + 1195], &Pair::new([0; 2], [0; 2], 0));
        write_sums(row, state, 0, [0; 2]);
        return;
    }
    write_active(row, input, state, step);
}

fn write_active(row: &mut [Value], input: &DotInputs, state: &mut [u16; 256], step: usize) {
    let op = step / 128;
    let pair = step % 128;
    let left = input.left(op);
    let right = input.right(op);
    write_polynomials(row, left, right);
    let product = Pair::new(
        [left[2 * pair], left[2 * pair + 1]],
        [right[2 * pair], right[2 * pair + 1]],
        ntt_gamma(pair as u8).expect("fixed pair index"),
    );
    write_pair(&mut row[WORK..WORK + 1195], &product);
    write_sums(row, state, pair, product.sums.map(|sum| sum.0));
    if step % 384 == 383 {
        state.fill(0);
    }
}

fn write_polynomials(row: &mut [Value], left: &[u16; 256], right: &[u16; 256]) {
    for index in 0..256 {
        row[WORK + index] = Value::from_u16(left[index]);
        row[WORK + 256 + index] = Value::from_u16(right[index]);
    }
}

fn write_sums(row: &mut [Value], state: &mut [u16; 256], pair: usize, product: [u16; 2]) {
    for lane in 0..2 {
        let current = state[2 * pair + lane];
        let sum = current + product[lane];
        write_canonical(row, ACC_INPUT + 25 * lane, current);
        write_canonical(row, ACC_OUTPUT + 25 * lane, sum % Q);
        row[CARRY + lane] = Value::from_bool(sum >= Q);
        state[2 * pair + lane] = sum % Q;
    }
}
