// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COINS, KEY, RHO, WIDTH};
use crate::{
    Value,
    mlkem_host::{Encapsulation, Polynomial},
};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(values: &mut [Value], input: &Encapsulation, dots: &[Polynomial; 4]) {
    let matrix = input.encryption.matrix.concat();
    for row in values.as_chunks_mut::<WIDTH>().0 {
        polynomials(row, 0, &matrix);
        polynomials(row, 2304, &input.encryption.public_polynomials);
        polynomials(row, 3072, &input.encryption.r_hat);
        polynomials(row, 3840, dots);
        polynomials(row, 6161, &input.encryption.noise);
        polynomials(row, 8209, &input.encryption.u);
        polynomials(row, 8977, &[input.encryption.v]);
        for (index, &value) in input.encryption.message.iter().enumerate() {
            row[7953 + index] = Value::from_bool(value == 1665);
        }
        bits(row, RHO, &input.encryption.matrix_seed);
        bits(row, KEY, &input.shared_key);
        bits(row, COINS, &input.coins);
    }
}

fn polynomials(row: &mut [Value], start: usize, polynomials: &[Polynomial]) {
    for (index, &value) in polynomials.iter().flatten().enumerate() {
        row[start + index] = Value::from_u16(value);
    }
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8; 32]) {
    for bit in 0..256 {
        row[start + bit] = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
