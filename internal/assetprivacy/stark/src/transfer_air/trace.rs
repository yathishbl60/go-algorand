// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    TransferAir,
    witness::{TransferWitness, amount},
};
use crate::{Value, amounts::Amounts};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

type Trace = RowMajorMatrix<Value>;

impl TransferAir {
    /// Generates private columns only; this does not verify ciphertexts or ledger state.
    pub fn trace(&self, witness: &TransferWitness<'_>) -> Option<RowMajorMatrix<Value>> {
        let input = witness.inputs.each_ref().map(|value| amount(value.opening));
        let output = witness.outputs.each_ref().map(amount);
        let amounts = Amounts::witness(input, output, self.conversion)?;
        let traces = self.component_traces(witness)?;
        if traces
            .iter()
            .any(|trace| trace.height() != crate::MIN_TRACE_ROWS)
        {
            return None;
        }
        Some(self.join(&traces, amounts.carry))
    }

    fn component_traces(&self, witness: &TransferWitness<'_>) -> Option<[Trace; 4]> {
        let a = &witness.inputs[0];
        let b = &witness.inputs[1];
        Some([
            self.input.trace(a.opening, a.secret, a.path)?,
            self.input.trace(b.opening, b.secret, b.path)?,
            self.output.trace(&witness.outputs[0])?,
            self.output.trace(&witness.outputs[1])?,
        ])
    }

    fn join(&self, traces: &[Trace; 4], carry: [Value; 3]) -> Trace {
        let mut values = Value::zero_vec(crate::MIN_TRACE_ROWS * self.width());
        for (r, row) in values.chunks_exact_mut(self.width()).enumerate() {
            let mut start = 0;
            for trace in traces {
                let width = trace.width();
                row[start..start + width]
                    .copy_from_slice(&trace.values[r * width..(r + 1) * width]);
                start += width;
            }
            row[self.carry_start()..].copy_from_slice(&carry);
        }
        RowMajorMatrix::new(values, self.width())
    }
}
