// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, PublicStatement, ROWS, RecoveryGroupAir, WIDTH, fixed};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct GroupWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: Vec<Value>,
}

impl RecoveryGroupAir {
    pub fn trace(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
    ) -> Option<GroupWitness> {
        let prepared = self.prepare(input, statement)?;
        let public_values = self.public_values(statement)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_components(input, statement, &mut values)?;
        fixed::write(&mut values, &prepared);
        Some(GroupWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values,
        })
    }
}
