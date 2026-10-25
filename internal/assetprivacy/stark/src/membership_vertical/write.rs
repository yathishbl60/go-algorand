// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, CURRENT, DIGEST, DIR, LEAF, STAGE_ROWS, WIDTH, witness::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub(super) fn fixed(
    leaf: [u8; 64],
    input: &Prepared,
    values: &mut [Value],
    target: (usize, usize, usize),
    rows: usize,
) {
    let leaf = digest_values(leaf);
    for (index, parent) in values
        .chunks_exact_mut(target.0)
        .skip(target.2)
        .take(rows)
        .enumerate()
    {
        let row = &mut parent[target.1..target.1 + WIDTH];
        row.fill(Value::ZERO);
        row[LEAF..LEAF + 32].copy_from_slice(&leaf);
        write_row(row, input, index);
    }
}

fn write_row(row: &mut [Value], input: &Prepared, index: usize) {
    if index < ACTIVE_ROWS {
        let stage = &input.stages[index / STAGE_ROWS];
        row[CURRENT..CURRENT + 32].copy_from_slice(&digest_values(stage.before));
        row[DIGEST..DIGEST + 32].copy_from_slice(&digest_values(stage.digest));
        row[DIR] = Value::from_bool(stage.direction);
    } else {
        row[CURRENT..CURRENT + 32].copy_from_slice(&digest_values(input.root));
    }
}
