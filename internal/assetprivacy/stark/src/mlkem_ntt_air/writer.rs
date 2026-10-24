// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    INPUT, MlKemNttAir, OUTPUT, STATE, WIDTH, program::Operation, witness::Arithmetic, write,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl MlKemNttAir {
    pub(crate) fn write_into(
        &self,
        input: [u16; 256],
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) -> Option<[u16; 256]> {
        self.check_target(values.len(), layout)?;
        let output = self.values(input)?;
        self.write_rows(input, output, values, layout);
        Some(output)
    }

    fn check_target(&self, len: usize, layout: (usize, usize, usize)) -> Option<()> {
        let (width, start, first) = layout;
        if width == 0 || !len.is_multiple_of(width) || start.checked_add(WIDTH)? > width {
            return None;
        }
        if first.checked_add(self.trace_rows())? > len / width {
            return None;
        }
        Some(())
    }

    fn write_rows(
        &self,
        input: [u16; 256],
        output: [u16; 256],
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) {
        let (width, start, first) = layout;
        let mut state = input;
        let rows = values
            .chunks_exact_mut(width)
            .skip(first)
            .take(self.trace_rows());
        let io = (&input, &output);
        for (step, parent) in rows.enumerate() {
            self.write_step(&mut parent[start..start + WIDTH], &mut state, io, step);
        }
    }
    fn write_step(
        &self,
        row: &mut [Value],
        state: &mut [u16; 256],
        io: (&[u16; 256], &[u16; 256]),
        step: usize,
    ) {
        let op = self.operation(step);
        let arithmetic = self.arithmetic(state, op.as_ref());
        fixed(row, state, io.0, io.1);
        write::arithmetic(row, &arithmetic);
        advance(state, op.as_ref(), &arithmetic);
    }
}

pub(super) fn advance(state: &mut [u16; 256], op: Option<&Operation>, value: &Arithmetic) {
    if let Some(op) = op {
        state[op.left] = value.low;
        if let Some(right) = op.right {
            state[right] = value.high;
        }
    }
}

fn fixed(row: &mut [Value], state: &[u16; 256], input: &[u16; 256], output: &[u16; 256]) {
    for index in 0..256 {
        row[STATE + index] = Value::from_u16(state[index]);
        row[INPUT + index] = Value::from_u16(input[index]);
        row[OUTPUT + index] = Value::from_u16(output[index]);
    }
}
