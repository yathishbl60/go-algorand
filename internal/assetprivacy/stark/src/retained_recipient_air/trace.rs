// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    PUBLIC_VALUES, ROWS, RetainedInput, RetainedRecipientAir, VIEW_ROW, WIDTH, WORK, fixed,
};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct RetainedWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
    pub envelope: [u8; 1629],
}

impl RetainedRecipientAir {
    pub fn public_values(
        &self,
        envelope: &[u8; 1629],
        current_root: [u8; 64],
    ) -> Option<[Value; PUBLIC_VALUES]> {
        let envelope = self.envelope.public_values(envelope)?;
        let root = digest_values(current_root);
        Some(core::array::from_fn(|i| match i {
            0..1068 => envelope[i],
            _ => root[i - 1068],
        }))
    }
    pub fn trace(
        &self,
        input: RetainedInput<'_>,
        current_root: [u8; 64],
    ) -> Option<RetainedWitness> {
        self.check_inputs(&input, current_root)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let envelope = self
            .envelope
            .write_into(input.envelope, &mut values, (WIDTH, WORK, 0))?;
        let shape = (WIDTH, WORK, VIEW_ROW);
        self.view
            .write_into(input.view(), current_root, &mut values, shape)?;
        fixed::write(&mut values, &input);
        Some(RetainedWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(&envelope, current_root)?,
            envelope,
        })
    }
    fn check_inputs(&self, input: &RetainedInput<'_>, root: [u8; 64]) -> Option<()> {
        self.envelope.check_inputs(&input.envelope)?;
        self.view.check_inputs(input.view(), root)?;
        Some(())
    }
}
