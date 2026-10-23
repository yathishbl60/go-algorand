// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LEFT, MlKemProductAir, OUTPUT, RIGHT, ROWS, WIDTH, witness::Pair, write};
use crate::{
    Value,
    mlkem_field::{Q, ntt_gamma},
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemProductAir {
    /// Generates private columns only; it never evaluates constraints or proves a KEM.
    pub fn trace(&self, left: [u16; 256], right: [u16; 256]) -> Option<RowMajorMatrix<Value>> {
        if left.iter().chain(&right).any(|&v| v >= Q) {
            return None;
        }
        let output = product(&left, &right);
        let mut values = Value::zero_vec(ROWS * WIDTH);
        for (step, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            for index in 0..256 {
                row[LEFT + index] = Value::from_u16(left[index]);
                row[RIGHT + index] = Value::from_u16(right[index]);
                row[OUTPUT + index] = Value::from_u16(output[index]);
            }
            write::pair(row, &pair(&left, &right, step));
        }
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}

fn pair(left: &[u16; 256], right: &[u16; 256], index: usize) -> Pair {
    if index >= 128 {
        return Pair::new([0; 2], [0; 2], 0);
    }
    let l = [left[2 * index], left[2 * index + 1]];
    let r = [right[2 * index], right[2 * index + 1]];
    Pair::new(l, r, ntt_gamma(index as u8).expect("fixed pair index"))
}

fn product(left: &[u16; 256], right: &[u16; 256]) -> [u16; 256] {
    let mut output = [0; 256];
    for index in 0..128 {
        let value = pair(left, right, index);
        output[2 * index] = value.sums[0].0;
        output[2 * index + 1] = value.sums[1].0;
    }
    output
}
