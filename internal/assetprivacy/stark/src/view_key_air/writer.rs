// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CANONICAL, COMMIT, NONZERO, ROWS, ViewKeyAir, WIDTH, prepared::Prepared};
use crate::{Value, mlkem_field::Canonical, sha3_air::digest_values};

impl ViewKeyAir {
    pub(crate) fn write_into(
        &self,
        key: &[u8],
        account: [u8; 32],
        epoch: u64,
        created: u64,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        check(values.len(), shape)?;
        let input = Prepared::new(self, (key, account, epoch, created))?;
        let (stride, start, first) = shape;
        let target = &mut values[first * stride..(first + ROWS) * stride];
        self.hash
            .write_into(&input.message, target, stride, start)?;
        write_auxiliary(target, (stride, start), &input);
        Some(input.commitment)
    }
}

fn check(len: usize, shape: (usize, usize, usize)) -> Option<()> {
    let (stride, start, first) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(ROWS)? <= len / stride).then_some(())
}

fn write_auxiliary(values: &mut [Value], shape: (usize, usize), input: &Prepared) {
    let digest = digest_values(input.commitment);
    for (index, parent) in values.chunks_exact_mut(shape.0).enumerate() {
        let row = &mut parent[shape.1..shape.1 + WIDTH];
        row[COMMIT..COMMIT + 32].copy_from_slice(&digest);
        row[NONZERO..NONZERO + 2].copy_from_slice(&input.inverses);
        let coefficient = input.coefficients.get(index).copied().unwrap_or(0);
        write_canonical(row, coefficient);
    }
}

fn write_canonical(row: &mut [Value], coefficient: u16) {
    let value = Canonical::witness(coefficient).expect("checked canonical key coefficient");
    row[CANONICAL] = value.value;
    row[CANONICAL + 1..CANONICAL + 13].copy_from_slice(&value.bits);
    row[CANONICAL + 13..CANONICAL + 25].copy_from_slice(&value.gap);
}
