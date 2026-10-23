// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir, MembershipPath};
use sha3::{Digest, Sha3_512};

impl MembershipAir {
    pub(super) fn message(&self, level: usize, child: [u8; 64], path: &MembershipPath) -> Vec<u8> {
        let mut message = self.prefix.clone();
        message.push(level as u8);
        let sibling = path.siblings[level];
        let (left, right) = if path.index & (1 << level) == 0 {
            (child, sibling)
        } else {
            (sibling, child)
        };
        message.extend(left);
        message.extend(right);
        message
    }

    /// Calculates the root; root acceptance and nullifier freshness are ledger checks.
    pub fn root(&self, leaf: [u8; 64], path: &MembershipPath) -> [u8; 64] {
        (0..DEPTH).fold(leaf, |child, level| {
            Sha3_512::digest(self.message(level, child, path)).into()
        })
    }
}
