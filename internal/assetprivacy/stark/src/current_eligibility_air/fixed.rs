// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, LEAF, REVISION, UPDATED, WIDTH, prepared::Prepared};
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
        bits(row, ACCOUNT, &input.input.account);
        bits(row, REVISION, &input.input.revision.to_be_bytes());
        bits(row, UPDATED, &input.input.updated.to_be_bytes());
        row[LEAF..LEAF + 32].copy_from_slice(&digest);
    }
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8]) {
    for bit in 0..bytes.len() * 8 {
        row[start + bit] = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
