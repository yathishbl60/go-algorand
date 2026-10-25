// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FRAME_ROWS, MlKemStreamMatrixAir, WIDTH};
use crate::Value;

impl MlKemStreamMatrixAir {
    pub(super) fn write_sized(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
        rows: usize,
    ) -> Option<[u16; 256]> {
        check(values.len(), target, rows)?;
        let input = self.prepare(seed)?;
        let output = input.coefficients;
        self.write_rows(values, target, input, rows);
        Some(output)
    }
}

fn check(len: usize, target: (usize, usize, usize), rows: usize) -> Option<()> {
    let (stride, start, first) = target;
    if rows < FRAME_ROWS || stride == 0 || !len.is_multiple_of(stride) {
        return None;
    }
    if start.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(rows)? <= len / stride).then_some(())
}
