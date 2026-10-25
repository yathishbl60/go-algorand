// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, EligibleRecordAir, NONZERO, ROWS, WIDTH, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};

impl EligibleRecordAir {
    pub(crate) fn write_into(
        &self,
        account: [u8; 32],
        revision: u64,
        updated: u64,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        check(values.len(), shape)?;
        let input = Prepared::new(self, account, revision, updated)?;
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
    for parent in values.chunks_exact_mut(shape.0) {
        let row = &mut parent[shape.1..shape.1 + WIDTH];
        row[COMMIT..COMMIT + 32].copy_from_slice(&digest);
        row[NONZERO..NONZERO + 2].copy_from_slice(&input.inverses);
    }
}
