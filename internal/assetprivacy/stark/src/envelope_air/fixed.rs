// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, COMMIT, EPOCH, KEY, NONZERO, OPENING, RHO, SHARED, WIDTH, prepared::Prepared};
use crate::Value;
use p3_field::{Field, PrimeCharacteristicRing};

pub(super) fn write(
    values: &mut [Value],
    input: &Prepared<'_>,
    commitment: [u8; 64],
    shape: (usize, usize, usize),
    rows: usize,
) {
    let inverse = Value::from_u32(input.input.epoch.count_ones()).inverse();
    let (stride, column, first) = shape;
    for parent in values.chunks_exact_mut(stride).skip(first).take(rows) {
        write_row(
            &mut parent[column..column + WIDTH],
            input,
            commitment,
            inverse,
        );
    }
}

fn write_row(row: &mut [Value], input: &Prepared<'_>, commitment: [u8; 64], inverse: Value) {
    coefficients(row, &input.kem.encryption.public_polynomials);
    bits(row, RHO, &input.kem.encryption.matrix_seed);
    bits(row, SHARED, &input.kem.shared_key);
    bits(row, AAD, &input.associated);
    bits(row, KEY, &input.key);
    bits(row, EPOCH, &input.input.epoch.to_be_bytes());
    row[NONZERO] = inverse;
    bits(row, OPENING, input.input.opening);
    for (index, bytes) in commitment.as_chunks::<2>().0.iter().enumerate() {
        row[COMMIT + index] = Value::from_u16(u16::from_le_bytes(*bytes));
    }
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8]) {
    for (index, value) in row[start..start + bytes.len() * 8].iter_mut().enumerate() {
        *value = Value::from_bool(bytes[index / 8] & (1 << (index % 8)) != 0);
    }
}

fn coefficients(row: &mut [Value], polynomials: &[[u16; 256]; 3]) {
    for (index, &value) in polynomials.iter().flatten().enumerate() {
        row[index] = Value::from_u16(value);
    }
}
