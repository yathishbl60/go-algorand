// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    FRAME_ROWS, SpendInput, SpendableNoteAir, WIDTH, fixed, prepared::Prepared, schedule,
    write_children,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl SpendableNoteAir {
    pub(crate) fn write_into(
        &self,
        input: SpendInput<'_>,
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        check(values.len(), shape, WIDTH)?;
        let input = Prepared::new(self, input, note_root, eligibility_root)?;
        self.write_prepared(&input, eligibility_root, values, shape)?;
        Some(input.nullifier)
    }
    pub(super) fn write_prepared(
        &self,
        input: &Prepared<'_>,
        eligibility_root: [u8; 64],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        check(values.len(), shape, WIDTH)?;
        clear(values, shape);
        write_children::write(self, input, eligibility_root, values, shape)?;
        fixed::write(values, input, shape, FRAME_ROWS);
        Some(())
    }
    pub(crate) fn write_preprocessed_into(
        &self,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        schedule::write(self, values, shape)
    }
}

pub(super) fn check(len: usize, shape: (usize, usize, usize), width: usize) -> Option<()> {
    let (stride, start, first) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(width)? > stride {
        return None;
    }
    (first.checked_add(FRAME_ROWS)? <= len / stride).then_some(())
}

fn clear(values: &mut [Value], shape: (usize, usize, usize)) {
    for row in values
        .chunks_exact_mut(shape.0)
        .skip(shape.2)
        .take(FRAME_ROWS)
    {
        row[shape.1..shape.1 + WIDTH].fill(Value::ZERO);
    }
}
