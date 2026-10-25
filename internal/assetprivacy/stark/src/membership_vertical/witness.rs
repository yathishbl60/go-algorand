// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MembershipVerticalAir;
use crate::membership_air::MembershipPath;
use sha3::{Digest, Sha3_512};

pub(super) struct Stage {
    pub message: Vec<u8>,
    pub before: [u8; 64],
    pub digest: [u8; 64],
    pub direction: bool,
}

pub(super) struct Prepared {
    pub stages: [Stage; 32],
    pub root: [u8; 64],
}

impl MembershipVerticalAir {
    pub(super) fn prepare(&self, leaf: [u8; 64], path: &MembershipPath) -> Prepared {
        let mut child = leaf;
        let stages = core::array::from_fn(|level| {
            let message = self.message(level, child, path);
            let digest = Sha3_512::digest(&message).into();
            let direction = path.index & (1 << level) != 0;
            let stage = Stage {
                message,
                before: child,
                digest,
                direction,
            };
            child = digest;
            stage
        });
        Prepared {
            stages,
            root: child,
        }
    }

    fn message(&self, level: usize, child: [u8; 64], path: &MembershipPath) -> Vec<u8> {
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
}
