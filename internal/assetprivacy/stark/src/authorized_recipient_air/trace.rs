// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AuthorizedRecipientAir, PUBLIC_VALUES, ROWS, RecipientInput, SPEND_ROW, VIEW_ROW, WIDTH, WORK,
    fixed,
};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct RecipientWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
    pub envelope: [u8; 1629],
}

impl AuthorizedRecipientAir {
    pub fn public_values(
        &self,
        envelope: &[u8; 1629],
        current_root: [u8; 64],
    ) -> Option<[Value; PUBLIC_VALUES]> {
        let envelope = self.envelope.public_values(envelope)?;
        let root = digest_values(current_root);
        Some(core::array::from_fn(|i| {
            if i < 1068 {
                envelope[i]
            } else {
                root[i - 1068]
            }
        }))
    }
    pub fn trace(
        &self,
        input: RecipientInput<'_>,
        current_root: [u8; 64],
    ) -> Option<RecipientWitness> {
        self.check_inputs(&input, current_root)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let envelope = self
            .envelope
            .write_into(input.envelope, &mut values, (WIDTH, WORK, 0))?;
        self.write_keys(&input, current_root, &mut values)?;
        fixed::write(&mut values, &input);
        Some(RecipientWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(&envelope, current_root)?,
            envelope,
        })
    }
    fn write_keys(
        &self,
        input: &RecipientInput<'_>,
        root: [u8; 64],
        values: &mut [Value],
    ) -> Option<()> {
        for (air, record, first) in [
            (&self.view, input.view(), VIEW_ROW),
            (&self.spend, input.spend(), SPEND_ROW),
        ] {
            let shape = (WIDTH, WORK, first);
            air.write_into(record, root, values, shape)?;
        }
        Some(())
    }

    fn check_inputs(&self, input: &RecipientInput<'_>, current_root: [u8; 64]) -> Option<()> {
        self.envelope.check_inputs(&input.envelope)?;
        self.view.check_inputs(input.view(), current_root)?;
        self.spend.check_inputs(input.spend(), current_root)?;
        Some(())
    }
}
