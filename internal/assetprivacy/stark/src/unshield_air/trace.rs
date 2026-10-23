// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{InputWitness, UnshieldAir};
use crate::{
    MIN_TRACE_ROWS, Value,
    amounts::{Amounts, PublicValue},
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl UnshieldAir {
    pub fn trace(&self, inputs: &[InputWitness<'_>; 2]) -> Option<RowMajorMatrix<Value>> {
        let carry = self.carry(inputs)?;
        let mut trace =
            RowMajorMatrix::new(Value::zero_vec(MIN_TRACE_ROWS * self.width()), self.width());
        for (index, input) in inputs.iter().enumerate() {
            let component = self.input.trace(input.opening, input.secret, input.path)?;
            if component.height() != MIN_TRACE_ROWS {
                return None;
            }
            self.copy_input(&mut trace, &component, index);
        }
        for row in trace.values.chunks_exact_mut(self.width()) {
            row[self.auxiliary_start() + 1..].copy_from_slice(&carry);
        }
        Some(trace)
    }

    fn carry(&self, inputs: &[InputWitness<'_>; 2]) -> Option<[Value; 3]> {
        let values = inputs.each_ref().map(|input| {
            u64::from_be_bytes(input.opening[169..177].try_into().expect("uint64 value"))
        });
        Some(
            Amounts::witness(
                values,
                [0, 0],
                PublicValue {
                    debit: 0,
                    credit: self.credit,
                },
            )?
            .carry,
        )
    }

    fn copy_input(
        &self,
        trace: &mut RowMajorMatrix<Value>,
        input: &RowMajorMatrix<Value>,
        index: usize,
    ) {
        let width = self.input.width();
        let start = index * width;
        for (i, row) in trace.values.chunks_exact_mut(self.width()).enumerate() {
            row[start..start + width].copy_from_slice(&input.values[i * width..(i + 1) * width]);
        }
    }
}
