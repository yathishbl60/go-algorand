// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    EnvelopeAir, EnvelopeInputs, FRAME_ROWS, WIDTH, fixed, prepared::Prepared, target, writer,
};
use crate::Value;

impl EnvelopeAir {
    pub(crate) fn check_inputs(&self, input: &EnvelopeInputs<'_>) -> Option<()> {
        Prepared::new(self, *input).map(|_| ())
    }

    /// Writes the complete active relation into one bounded parent rectangle.
    pub(crate) fn write_into(
        &self,
        input: EnvelopeInputs<'_>,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 1629]> {
        target::check(values.len(), shape, WIDTH, FRAME_ROWS)?;
        let prepared = Prepared::new(self, input)?;
        target::clear(values, shape, WIDTH, FRAME_ROWS);
        let envelope = writer::write(self, &prepared, values, shape)?;
        fixed::write(
            values,
            &prepared,
            self.context.commitment(),
            shape,
            FRAME_ROWS,
        );
        Some(envelope)
    }
}
