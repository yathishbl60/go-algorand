// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir, MembershipPath};
use crate::{Value, sha3_air::digest_values};
use p3_matrix::dense::RowMajorMatrix;
use sha3::{Digest, Sha3_512};

pub(super) struct Stage {
    pub hash: RowMajorMatrix<Value>,
    pub digest: [Value; 32],
    pub direction: bool,
}

impl MembershipAir {
    /// Allocates all 32 hashes horizontally; proving memory remains unmeasured.
    pub fn trace(&self, leaf: [u8; 64], path: &MembershipPath) -> Option<RowMajorMatrix<Value>> {
        let mut stages = Vec::with_capacity(DEPTH);
        let mut child = leaf;
        for level in 0..DEPTH {
            let message = self.message(level, child, path);
            child = Sha3_512::digest(&message).into();
            stages.push(Stage {
                hash: self.hash.trace(&message)?,
                digest: digest_values(child),
                direction: path.index & (1 << level) != 0,
            });
        }
        Some(self.join(stages, digest_values(leaf)))
    }
}
