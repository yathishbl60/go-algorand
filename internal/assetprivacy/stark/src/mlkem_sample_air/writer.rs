// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemSampleAir, trace::Prepared, witness::Step};
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;

impl MlKemSampleAir {
    /// Writes only this component's columns; failure leaves the target unchanged.
    pub(crate) fn write_into(
        &self,
        prefix: &[u8],
        values: &mut [Value],
        row_width: usize,
        start: usize,
    ) -> Option<()> {
        self.check_target(prefix.len(), values.len(), row_width, start)?;
        let input = self.prepare(prefix)?;
        self.write_rows(values, row_width, start, input);
        Some(())
    }

    fn check_target(&self, bytes: usize, len: usize, width: usize, start: usize) -> Option<()> {
        let component_width = <Self as BaseAir<Value>>::width(self);
        if bytes != self.prefix_bytes()
            || self.trace_rows().checked_mul(width)? != len
            || start.checked_add(component_width)? > width
        {
            return None;
        }
        Some(())
    }

    fn write_rows(&self, values: &mut [Value], width: usize, start: usize, input: Prepared) {
        let component_width = <Self as BaseAir<Value>>::width(self);
        let mut cursor = 0;
        for (index, target) in values.chunks_exact_mut(width).enumerate() {
            let row = &mut target[start..start + component_width];
            row.fill(Value::ZERO);
            row[..input.fixed.len()].copy_from_slice(&input.fixed);
            let step = Step {
                candidate: input.candidates.get(index).copied().unwrap_or(0),
                active: index < input.candidates.len(),
                cursor,
            };
            cursor += usize::from(self.write_step(row, step));
        }
    }
}
