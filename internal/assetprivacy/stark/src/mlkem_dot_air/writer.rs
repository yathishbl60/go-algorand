// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE, DotInputs, MlKemDotAir, Polynomial, ROWS, WIDTH, writer_step};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl MlKemDotAir {
    pub(crate) fn write_into(
        &self,
        input: &DotInputs,
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) -> Option<[Polynomial; 4]> {
        check_target(values.len(), layout)?;
        let output = self.values(input)?;
        self.write_prepared(input, output, values, layout);
        Some(output)
    }
    pub(super) fn write_prepared(
        &self,
        input: &DotInputs,
        output: [Polynomial; 4],
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) {
        let fixed = fixed(input, &output);
        let mut state = [0; 256];
        for (step, target) in values
            .chunks_exact_mut(layout.0)
            .skip(layout.2)
            .take(ROWS)
            .enumerate()
        {
            let row = &mut target[layout.1..layout.1 + WIDTH];
            prepare_row(row, &fixed);
            writer_step::write(row, input, &mut state, step);
        }
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

fn fixed(input: &DotInputs, output: &[Polynomial; 4]) -> Vec<Value> {
    let mut values = Value::zero_vec(CORE);
    let polynomials = input
        .matrix
        .iter()
        .flatten()
        .chain(&input.public_polynomials)
        .chain(&input.r_hat)
        .chain(output);
    for (index, polynomial) in polynomials.enumerate() {
        for (coefficient, &value) in polynomial.iter().enumerate() {
            values[index * 256 + coefficient] = Value::from_u16(value);
        }
    }
    values
}

fn prepare_row(row: &mut [Value], fixed: &[Value]) {
    row.fill(Value::ZERO);
    row[..CORE].copy_from_slice(fixed);
}
