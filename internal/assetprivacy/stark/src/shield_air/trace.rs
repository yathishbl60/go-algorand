// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ShieldAir;
use crate::{
    MIN_TRACE_ROWS, Value,
    amounts::{Amounts, PublicValue},
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl ShieldAir {
    pub fn trace(&self, openings: &[[u8; 512]; 2]) -> Option<RowMajorMatrix<Value>> {
        let carry = self.carry(openings)?;
        let a = self.output.trace(&openings[0])?;
        let b = self.output.trace(&openings[1])?;
        if a.height() != MIN_TRACE_ROWS || b.height() != MIN_TRACE_ROWS {
            return None;
        }
        Some(self.join(a, b, carry))
    }

    fn carry(&self, openings: &[[u8; 512]; 2]) -> Option<[Value; 3]> {
        let output = openings
            .each_ref()
            .map(|opening| u64::from_be_bytes(opening[169..177].try_into().expect("uint64 value")));
        Some(
            Amounts::witness(
                [0, 0],
                output,
                PublicValue {
                    debit: self.debit,
                    credit: 0,
                },
            )?
            .carry,
        )
    }

    fn join(
        &self,
        a: RowMajorMatrix<Value>,
        b: RowMajorMatrix<Value>,
        carry: [Value; 3],
    ) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(MIN_TRACE_ROWS * self.width());
        let width = self.output.width();
        for (i, row) in values.chunks_exact_mut(self.width()).enumerate() {
            row[..width].copy_from_slice(&a.values[i * width..(i + 1) * width]);
            row[width..2 * width].copy_from_slice(&b.values[i * width..(i + 1) * width]);
            row[self.auxiliary_start() + 1..].copy_from_slice(&carry);
        }
        RowMajorMatrix::new(values, self.width())
    }
}
