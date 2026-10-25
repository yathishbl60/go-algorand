// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, COMMIT, INVERSE, OWNER, SECRET, SERIAL, VALUE, WIDTH, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::{Field, PrimeCharacteristicRing};

pub(super) fn write(
    values: &mut [Value],
    input: &Prepared<'_>,
    shape: (usize, usize, usize),
    rows: usize,
) {
    let owner = digest_values(input.input.opening[73..137].try_into().expect("owner"));
    let commitment = digest_values(input.commitment);
    let inverse = inverse(input.input.secret);
    for parent in values.chunks_exact_mut(shape.0).skip(shape.2).take(rows) {
        let row = &mut parent[shape.1..shape.1 + WIDTH];
        bits(row, ACCOUNT, &input.input.opening[137..169]);
        value_bits(row, input.input.value());
        bits(row, SERIAL, &input.input.opening[177..209]);
        bits(row, SECRET, &input.input.secret);
        row[OWNER..COMMIT].copy_from_slice(&owner);
        row[COMMIT..INVERSE].copy_from_slice(&commitment);
        row[INVERSE] = inverse;
    }
}

fn inverse(secret: [u8; 32]) -> Value {
    let count: u32 = secret.iter().map(|byte| byte.count_ones()).sum();
    Value::from_u32(count).inverse()
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8]) {
    for bit in 0..bytes.len() * 8 {
        row[start + bit] = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}

fn value_bits(row: &mut [Value], value: u64) {
    for bit in 0..64 {
        row[VALUE + bit] = Value::from_bool(value & (1 << bit) != 0);
    }
}
