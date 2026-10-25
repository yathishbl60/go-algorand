// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, CREATED, EPOCH, LEAF, OWNER, RHO, WIDTH, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(
    values: &mut [Value],
    input: &Prepared<'_>,
    shape: (usize, usize, usize),
    rows: usize,
) {
    let digest = digest_values(input.leaf);
    for parent in values.chunks_exact_mut(shape.0).skip(shape.2).take(rows) {
        let row = &mut parent[shape.1..shape.1 + WIDTH];
        for (target, &value) in row[..ACCOUNT].iter_mut().zip(&input.coefficients) {
            *target = Value::from_u16(value);
        }
        bits(row, ACCOUNT, &input.input.account);
        bits(row, EPOCH, &input.input.epoch.to_be_bytes());
        bits(row, CREATED, &input.input.created.to_be_bytes());
        bits(row, RHO, &input.rho);
        bits(row, OWNER, &input.owner);
        row[LEAF..LEAF + 32].copy_from_slice(&digest);
    }
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8]) {
    for bit in 0..bytes.len() * 8 {
        row[start + bit] = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
