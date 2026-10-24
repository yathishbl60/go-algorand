// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EnvelopeAir, PUBLIC_VALUES, ROWS, WIDTH, fixed, prepared::Prepared, writer};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

/// Secret prover inputs. m and GCM nonce must be generated with fresh entropy.
pub struct EnvelopeInputs<'a> {
    pub public_key: &'a [u8],
    pub message: [u8; 32],
    pub epoch: u64,
    pub nonce: [u8; 12],
    pub opening: &'a [u8; 512],
}

pub struct EnvelopeWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
    pub envelope: [u8; 1629],
}

impl EnvelopeAir {
    /// Constructs witnesses only. Canonical key/note checks precede allocation.
    pub fn trace(&self, input: EnvelopeInputs<'_>) -> Option<EnvelopeWitness> {
        let prepared = Prepared::new(self, input)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let envelope = writer::write(self, &prepared, &mut values)?;
        fixed::write(&mut values, &prepared, self.context.commitment());
        Some(EnvelopeWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(&envelope)?,
            envelope,
        })
    }
}
