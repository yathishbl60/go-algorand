// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MembershipVerticalAir, ROWS, WIDTH};
use crate::{Value, membership_air::MembershipPath, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl MembershipVerticalAir {
    pub fn root(&self, leaf: [u8; 64], path: &MembershipPath) -> [u8; 64] {
        self.prepare(leaf, path).root
    }

    pub fn public_values(&self, root: [u8; 64]) -> [Value; 32] {
        digest_values(root)
    }

    /// Constructs secret columns only; does not evaluate constraints or prove.
    pub fn trace(&self, leaf: [u8; 64], path: &MembershipPath) -> Option<RowMajorMatrix<Value>> {
        let input = self.prepare(leaf, path);
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.write_rows(leaf, &input, &mut values, (WIDTH, 0, 0), ROWS)?;
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}
