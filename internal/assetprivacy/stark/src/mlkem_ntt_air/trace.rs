// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    INPUT, MlKemNttAir, OUTPUT, STATE, WIDTH, program::Operation, witness::Arithmetic, write,
};
use crate::{Value, mlkem_field::Q};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemNttAir {
    /// Private witness generation only. Noncanonical input coefficients are rejected.
    pub fn trace(&self, input: [u16; 256]) -> Option<RowMajorMatrix<Value>> {
        let output = self.values(input)?;
        let mut state = input;
        let mut values = Value::zero_vec(self.trace_rows() * WIDTH);
        for (step, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            let op = self.operation(step);
            let arithmetic = self.arithmetic(&state, op.as_ref());
            write_fixed(row, &input, &output);
            for (index, value) in state.into_iter().enumerate() {
                row[STATE + index] = Value::from_u16(value);
            }
            write::arithmetic(row, &arithmetic);
            advance(&mut state, op.as_ref(), &arithmetic);
        }
        Some(RowMajorMatrix::new(values, WIDTH))
    }

    /// Computes the same witness arithmetic without allocating a trace matrix.
    pub fn values(&self, input: [u16; 256]) -> Option<[u16; 256]> {
        if input.iter().any(|&value| value >= Q) {
            return None;
        }
        let mut state = input;
        for step in 0..self.trace_rows() {
            let op = self.operation(step);
            let arithmetic = self.arithmetic(&state, op.as_ref());
            advance(&mut state, op.as_ref(), &arithmetic);
        }
        Some(state)
    }
}

fn advance(state: &mut [u16; 256], op: Option<&Operation>, value: &Arithmetic) {
    if let Some(op) = op {
        state[op.left] = value.low;
        if let Some(right) = op.right {
            state[right] = value.high;
        }
    }
}

fn write_fixed(row: &mut [Value], input: &[u16; 256], output: &[u16; 256]) {
    for index in 0..256 {
        row[INPUT + index] = Value::from_u16(input[index]);
        row[OUTPUT + index] = Value::from_u16(output[index]);
    }
}
