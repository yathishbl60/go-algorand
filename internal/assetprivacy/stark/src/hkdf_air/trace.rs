// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HkdfAir, INFO};
use crate::{MIN_TRACE_ROWS, Value};
use p3_field::{PrimeCharacteristicRing, PrimeField64};
use p3_matrix::{Matrix, dense::RowMajorMatrix};

type Trace = RowMajorMatrix<Value>;

impl HkdfAir {
    pub fn trace(&self, shared: &[u8; 32], salt: &[u8; 64]) -> Option<Trace> {
        let mut trace = Trace::new(Value::zero_vec(MIN_TRACE_ROWS * self.width()), self.width());
        let prk = self.extract_trace(&mut trace, shared, salt)?;
        let mut info = INFO.to_vec();
        info.push(1);
        let expanded = self.expand.trace(&prk, &info)?;
        self.copy_component(&mut trace, &expanded, self.extract.width())?;
        Some(trace)
    }

    fn extract_trace(
        &self,
        trace: &mut Trace,
        shared: &[u8; 32],
        salt: &[u8; 64],
    ) -> Option<[u8; 64]> {
        let extracted = self.extract.trace(salt, shared)?;
        self.copy_component(trace, &extracted, 0)?;
        let bits = self
            .extract
            .output_bits(&extracted.values[..self.extract.width()])?;
        Some(bits.map(|byte| {
            byte.iter().enumerate().fold(0u8, |value, (bit, v)| {
                value | ((v.as_canonical_u64() as u8) << bit)
            })
        }))
    }

    fn copy_component(&self, trace: &mut Trace, component: &Trace, start: usize) -> Option<()> {
        if component.height() != MIN_TRACE_ROWS {
            return None;
        }
        let width = component.width();
        for (row, destination) in trace.values.chunks_exact_mut(self.width()).enumerate() {
            destination[start..start + width]
                .copy_from_slice(&component.values[row * width..(row + 1) * width]);
        }
        Some(())
    }
}
