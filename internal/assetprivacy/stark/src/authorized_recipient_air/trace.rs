// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AuthorizedRecipientAir, PUBLIC_VALUES, ROWS, RecipientInput, WIDTH, WORK, fixed};
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
        current_eligibility_root: [u8; 64],
    ) -> Option<[Value; PUBLIC_VALUES]> {
        let envelope = self.envelope.public_values(envelope)?;
        let keys = digest_values(current_root);
        let eligibility = digest_values(current_eligibility_root);
        Some(core::array::from_fn(|i| match i {
            0..1068 => envelope[i],
            1068..1100 => keys[i - 1068],
            _ => eligibility[i - 1100],
        }))
    }
    pub fn trace(
        &self,
        input: RecipientInput<'_>,
        current_root: [u8; 64],
        current_eligibility_root: [u8; 64],
    ) -> Option<RecipientWitness> {
        self.check_inputs(&input, current_root, current_eligibility_root)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let envelope = self
            .envelope
            .write_into(input.envelope, &mut values, (WIDTH, WORK, 0))?;
        self.write_authorization(&input, current_root, current_eligibility_root, &mut values)?;
        fixed::write(&mut values, &input);
        self.finish(envelope, current_root, current_eligibility_root, values)
    }
    fn finish(
        &self,
        envelope: [u8; 1629],
        keys: [u8; 64],
        eligibility: [u8; 64],
        values: Vec<Value>,
    ) -> Option<RecipientWitness> {
        Some(RecipientWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(&envelope, keys, eligibility)?,
            envelope,
        })
    }
    fn check_inputs(
        &self,
        input: &RecipientInput<'_>,
        keys: [u8; 64],
        eligibility: [u8; 64],
    ) -> Option<()> {
        self.envelope.check_inputs(&input.envelope)?;
        self.view.check_inputs(input.view(), keys)?;
        self.spend.check_inputs(input.spend(), keys)?;
        self.eligibility
            .check_inputs(input.eligibility(), eligibility)?;
        Some(())
    }
}
