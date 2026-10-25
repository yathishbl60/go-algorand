// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EligibleRecordAir, ROWS, WIDTH, prepared::Prepared};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl EligibleRecordAir {
    pub fn trace(
        &self,
        account: [u8; 32],
        revision: u64,
        updated: u64,
    ) -> Option<RowMajorMatrix<Value>> {
        Prepared::new(self, account, revision, updated)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_into(account, revision, updated, &mut values, (WIDTH, 0, 0))?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }
    /// Canonical unfrozen snapshot hash only; allocates no trace.
    pub fn commitment(&self, account: [u8; 32], revision: u64, updated: u64) -> Option<[u8; 64]> {
        Some(Prepared::new(self, account, revision, updated)?.commitment)
    }
}
