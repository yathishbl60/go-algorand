// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir, MembershipPath};
use crate::{Value, sha3_air::digest_values};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use sha3::{Digest, Sha3_512};

pub(super) struct Stage {
    pub hash: RowMajorMatrix<Value>,
    pub digest: [Value; 32],
    pub direction: bool,
}

impl MembershipAir {
    /// Generates and drops one source hash at a time; proving memory is unmeasured.
    pub fn trace(&self, leaf: [u8; 64], path: &MembershipPath) -> Option<RowMajorMatrix<Value>> {
        let (first, mut child) = self.stage(0, leaf, path)?;
        let mut values = self.empty_trace(first.hash.height(), digest_values(leaf));
        self.copy_stage(&mut values, 0, &first)?;
        drop(first);
        for level in 1..DEPTH {
            let (stage, next) = self.stage(level, child, path)?;
            self.copy_stage(&mut values, level, &stage)?;
            child = next;
        }
        Some(RowMajorMatrix::new(values, self.width()))
    }

    fn stage(
        &self,
        level: usize,
        child: [u8; 64],
        path: &MembershipPath,
    ) -> Option<(Stage, [u8; 64])> {
        let message = self.message(level, child, path);
        let digest = Sha3_512::digest(&message).into();
        let stage = Stage {
            hash: self.hash.trace(&message)?,
            digest: digest_values(digest),
            direction: path.index & (1 << level) != 0,
        };
        Some((stage, digest))
    }
}
