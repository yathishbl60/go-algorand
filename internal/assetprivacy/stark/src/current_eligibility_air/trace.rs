// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CurrentEligibilityAir, PUBLIC_VALUES, ROWS, WIDTH, fixed, prepared::Prepared};
use crate::{Value, membership_air::MembershipPath, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

#[derive(Clone, Copy)]
pub struct EligibilityInput<'a> {
    pub account: [u8; 32],
    pub revision: u64,
    pub updated: u64,
    pub path: &'a MembershipPath,
}

pub struct EligibilityWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
}

impl CurrentEligibilityAir {
    pub fn public_values(&self, current_root: [u8; 64]) -> [Value; PUBLIC_VALUES] {
        digest_values(current_root)
    }
    pub(crate) fn check_inputs(&self, input: EligibilityInput<'_>, root: [u8; 64]) -> Option<()> {
        Prepared::new(self, input, root).map(|_| ())
    }
    pub fn trace(
        &self,
        input: EligibilityInput<'_>,
        current_root: [u8; 64],
    ) -> Option<EligibilityWitness> {
        let prepared = Prepared::new(self, input, current_root)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_prepared(&prepared, &mut values, (WIDTH, 0, 0))?;
        fixed::write(&mut values, &prepared, (WIDTH, 0, 0), ROWS);
        Some(EligibilityWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values: self.public_values(current_root),
        })
    }
}
