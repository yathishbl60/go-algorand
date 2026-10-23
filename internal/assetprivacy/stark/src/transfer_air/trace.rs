// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    TransferAir,
    witness::{TransferWitness, amount},
};
use crate::{MIN_TRACE_ROWS, Value, amounts::Amounts};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

type Trace = RowMajorMatrix<Value>;

impl TransferAir {
    /// Generates and drops each component separately; no ciphertext/state verification.
    pub fn trace(&self, witness: &TransferWitness<'_>) -> Option<Trace> {
        let carry = self.carry(witness)?;
        let mut trace =
            RowMajorMatrix::new(Value::zero_vec(MIN_TRACE_ROWS * self.width()), self.width());
        for index in 0..4 {
            let component = self.component_trace(witness, index)?;
            if component.height() != MIN_TRACE_ROWS {
                return None;
            }
            self.copy_component(&mut trace, &component, index);
        }
        for row in trace.values.chunks_exact_mut(self.width()) {
            row[self.carry_start()..].copy_from_slice(&carry);
        }
        Some(trace)
    }

    fn carry(&self, witness: &TransferWitness<'_>) -> Option<[Value; 3]> {
        let input = witness.inputs.each_ref().map(|value| amount(value.opening));
        let output = witness.outputs.each_ref().map(amount);
        Some(Amounts::witness(input, output, self.conversion)?.carry)
    }

    fn component_trace(&self, witness: &TransferWitness<'_>, index: usize) -> Option<Trace> {
        if index < 2 {
            let input = &witness.inputs[index];
            return self.input.trace(input.opening, input.secret, input.path);
        }
        self.output.trace(&witness.outputs[index - 2])
    }

    fn copy_component(&self, trace: &mut Trace, input: &Trace, index: usize) {
        let start = if index < 2 {
            index * self.input.width()
        } else {
            2 * self.input.width() + (index - 2) * self.output.width()
        };
        let width = input.width();
        for (i, row) in trace.values.chunks_exact_mut(self.width()).enumerate() {
            row[start..start + width].copy_from_slice(&input.values[i * width..(i + 1) * width]);
        }
    }
}
