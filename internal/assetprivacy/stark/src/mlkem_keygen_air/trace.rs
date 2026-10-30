// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::witness::Inputs;
use super::{
    CARRY, CORE_ROWS, DOT_ROWS, GROUP, MlKemKeygenAir, PublicStatement, ROWS, TRANSFORMED, WIDTH,
};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct GroupWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: Vec<Value>,
}

impl MlKemKeygenAir {
    pub fn trace(&self, seed: [u8; 32], statement: &PublicStatement) -> Option<GroupWitness> {
        let public_values = self.public_values(statement)?;
        let input = self.prepare(seed)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.core.write_frame(seed, &mut values, (WIDTH, 0, 0))?;
        self.dot
            .write_into(&input.dot, &mut values, (WIDTH, 0, CORE_ROWS))?;
        globals(&mut values, &input, statement.group);
        Some(GroupWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values,
        })
    }
}
fn globals(values: &mut [Value], input: &Inputs, group: [u8; 64]) {
    let group = digest_values(group);
    for (i, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
        for (cell, &value) in row[TRANSFORMED..GROUP]
            .iter_mut()
            .zip(input.transformed.iter().flatten())
        {
            *cell = Value::from_u16(value);
        }
        row[GROUP..WIDTH].copy_from_slice(&group);
        if (CORE_ROWS..CORE_ROWS + DOT_ROWS).contains(&i) {
            for (cell, &value) in row[CARRY..CARRY + 768].iter_mut().zip(&input.carry) {
                *cell = Value::from_u16(value);
            }
        }
    }
}
