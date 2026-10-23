// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemProductAir, ROWS, SCHEDULE};
use crate::{Value, mlkem_field::ntt_gamma};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemProductAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * SCHEDULE);
        for pair in 0..128 {
            values[pair * SCHEDULE + pair] = Value::ONE;
            values[pair * SCHEDULE + 128] =
                Value::from_u16(ntt_gamma(pair as u8).expect("fixed pair index"));
        }
        RowMajorMatrix::new(values, SCHEDULE)
    }
}
