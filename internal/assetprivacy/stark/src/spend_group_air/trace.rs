// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, PublicStatement, ROWS, SpendGroupAir, WIDTH, writer};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct GroupWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: Vec<Value>,
}

impl SpendGroupAir {
    /// Preflights canonical notes, both roots, exact public outputs and conservation.
    pub fn trace(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
    ) -> Option<GroupWitness> {
        let amounts = self.prepare(input, statement)?;
        let public_values = self.public_values(statement)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_components(input, statement, &mut values)?;
        writer::write_amounts(&mut values, &amounts);
        Some(GroupWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values,
        })
    }
}
