// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CurrentEligibilityAir, EligibilityInput, FRAME_ROWS, MEMBER_ROW, PREPROCESSED, WIDTH, WORK,
    fixed, prepared::Prepared, record_writer, schedule,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl CurrentEligibilityAir {
    pub(crate) fn write_into(
        &self,
        input: EligibilityInput<'_>,
        root: [u8; 64],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        check(values.len(), shape, WIDTH, FRAME_ROWS)?;
        let input = Prepared::new(self, input, root)?;
        self.write_prepared(&input, values, shape)
    }
    pub(super) fn write_prepared(
        &self,
        input: &Prepared<'_>,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        check(values.len(), shape, WIDTH, FRAME_ROWS)?;
        clear(values, shape, WIDTH, FRAME_ROWS);
        let child = (shape.0, shape.1 + WORK, shape.2);
        record_writer::write(self, input.input, values, child)?;
        let child = (shape.0, shape.1 + WORK, shape.2 + MEMBER_ROW);
        self.membership
            .write_into(input.leaf, input.input.path, values, child)?;
        fixed::write(values, input, shape, FRAME_ROWS);
        Some(())
    }
    pub(crate) fn write_preprocessed_into(
        &self,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        check(values.len(), shape, PREPROCESSED, FRAME_ROWS)?;
        schedule::write(self, values, shape)
    }
}

pub(super) fn check(
    len: usize,
    shape: (usize, usize, usize),
    width: usize,
    rows: usize,
) -> Option<()> {
    let (stride, start, first) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(width)? > stride {
        return None;
    }
    (first.checked_add(rows)? <= len / stride).then_some(())
}

fn clear(values: &mut [Value], shape: (usize, usize, usize), width: usize, rows: usize) {
    for row in values.chunks_exact_mut(shape.0).skip(shape.2).take(rows) {
        row[shape.1..shape.1 + width].fill(Value::ZERO);
    }
}
