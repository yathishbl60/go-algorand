// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE, END, MlKemDotAir, OP, PRE_GAMMA, ROWS, SCHEDULE_WIDTH};
use crate::{Value, mlkem_field::ntt_gamma};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MlKemDotAir {
    pub(super) fn schedule(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * SCHEDULE_WIDTH);
        for step in 0..ACTIVE {
            let pair = step % 128;
            let row = &mut values[step * SCHEDULE_WIDTH..(step + 1) * SCHEDULE_WIDTH];
            row[pair] = Value::ONE;
            row[OP + step / 128] = Value::ONE;
            row[PRE_GAMMA] = Value::from_u16(ntt_gamma(pair as u8).expect("fixed pair index"));
            if step % 384 == 383 {
                row[END + step / 384] = Value::ONE;
            }
        }
        RowMajorMatrix::new(values, SCHEDULE_WIDTH)
    }
}
