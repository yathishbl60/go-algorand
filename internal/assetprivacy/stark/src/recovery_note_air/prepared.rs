// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RecoveryInput, RecoveryNoteAir};
use sha3::{Digest, Sha3_512};

pub(super) struct Prepared<'a> {
    pub input: RecoveryInput<'a>,
    pub commitment: [u8; 64],
    pub message: [u8; 179],
    pub nullifier: [u8; 64],
}

impl<'a> Prepared<'a> {
    pub fn new(air: &RecoveryNoteAir, input: RecoveryInput<'a>, root: [u8; 64]) -> Option<Self> {
        let commitment = air.note.commitment(input.opening)?;
        if root == [0; 64] || air.membership.root(commitment, input.path) != root {
            return None;
        }
        let message = air.message(input.opening);
        Some(Self {
            input,
            commitment,
            message,
            nullifier: Sha3_512::digest(message).into(),
        })
    }
}
