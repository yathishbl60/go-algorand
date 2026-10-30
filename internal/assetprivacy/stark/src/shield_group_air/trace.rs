// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, GROUP, PublicStatement, ROWS, ShieldGroupAir, WIDTH, WORK};
use crate::{MIN_TRACE_ROWS, Value, amounts::Amounts, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct GroupWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: Vec<Value>,
}

impl ShieldGroupAir {
    pub fn trace(
        &self,
        openings: &[[u8; 512]; 2],
        statement: &PublicStatement,
    ) -> Option<GroupWitness> {
        let public_values = self.public_values(statement)?;
        let amounts = self.prepare(openings, statement)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        for (index, opening) in openings.iter().enumerate() {
            self.output
                .write_into(opening, &mut values, (WIDTH, WORK, index * MIN_TRACE_ROWS))?;
        }
        write_globals(&mut values, &amounts, statement.group);
        Some(GroupWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values,
        })
    }
    fn prepare(
        &self,
        openings: &[[u8; 512]; 2],
        statement: &PublicStatement,
    ) -> Option<Amounts<Value>> {
        for (opening, expected) in openings.iter().zip(statement.outputs) {
            if self.output.commitment(opening)? != expected {
                return None;
            }
        }
        let outgoing = openings
            .each_ref()
            .map(|opening| u64::from_be_bytes(opening[169..177].try_into().expect("amount")));
        Amounts::witness([0, 0], outgoing, self.conversion())
    }
}

fn write_globals(values: &mut [Value], amounts: &Amounts<Value>, group: [u8; 64]) {
    let context = digest_values(group);
    for row in values.as_chunks_mut::<WIDTH>().0 {
        for (index, bits) in amounts.values.iter().enumerate() {
            row[index * 64..index * 64 + 64].copy_from_slice(bits);
        }
        row[CARRY..GROUP].copy_from_slice(&amounts.carry);
        row[GROUP..WORK].copy_from_slice(&context);
    }
}
