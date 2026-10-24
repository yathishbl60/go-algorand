// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemCiphertextAir, ROWS, WIDTH, witness::Prepared};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl MlKemCiphertextAir {
    /// Writes exactly this frame after checking all bounds and coefficient inputs.
    pub(crate) fn write_into(
        &self,
        u: &[[u16; 256]; 3],
        v: &[u16; 256],
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) -> Option<[u8; 1088]> {
        check_target(values.len(), layout)?;
        let input = Prepared::new(u, v)?;
        write_rows(values, layout, &input);
        Some(input.ciphertext)
    }
}

fn write_rows(values: &mut [Value], layout: (usize, usize, usize), input: &Prepared) {
    let (width, start, first) = layout;
    for (index, target) in values
        .chunks_exact_mut(width)
        .skip(first)
        .take(ROWS)
        .enumerate()
    {
        let row = &mut target[start..start + WIDTH];
        row.fill(Value::ZERO);
        input.write(row, index);
    }
}

fn check_target(len: usize, layout: (usize, usize, usize)) -> Option<()> {
    let (width, start, first) = layout;
    if width == 0 || !len.is_multiple_of(width) || start.checked_add(WIDTH)? > width {
        return None;
    }
    if first.checked_add(ROWS)? > len / width {
        return None;
    }
    Some(())
}
