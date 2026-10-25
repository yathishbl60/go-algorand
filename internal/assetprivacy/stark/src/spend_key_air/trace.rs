// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ROWS, SpendKeyAir, WIDTH, prepared::Prepared};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl SpendKeyAir {
    pub fn trace(
        &self,
        owner: [u8; 64],
        account: [u8; 32],
        epoch: u64,
        created: u64,
    ) -> Option<RowMajorMatrix<Value>> {
        Prepared::new(self, (owner, account, epoch, created))?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_into(owner, account, epoch, created, &mut values, (WIDTH, 0, 0))?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }
    /// Canonical record commitment only, without trace allocation.
    pub fn commitment(
        &self,
        owner: [u8; 64],
        account: [u8; 32],
        epoch: u64,
        created: u64,
    ) -> Option<[u8; 64]> {
        Some(Prepared::new(self, (owner, account, epoch, created))?.commitment)
    }
}
