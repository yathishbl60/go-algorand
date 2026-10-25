// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SpendInput, SpendableNoteAir};
use sha3::{Digest, Sha3_512};

pub(super) struct Prepared<'a> {
    pub input: SpendInput<'a>,
    pub commitment: [u8; 64],
    pub owner_message: [u8; 109],
    pub nullifier_message: [u8; 245],
    pub nullifier: [u8; 64],
}

impl<'a> Prepared<'a> {
    pub fn new(
        air: &SpendableNoteAir,
        input: SpendInput<'a>,
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
    ) -> Option<Self> {
        let commitment = air.note.commitment(input.opening)?;
        let owner_message = air.owner_message(input.opening, input.secret)?;
        air.check_roots(input, commitment, note_root, eligibility_root)?;
        let nullifier_message =
            crate::nullifier_v2::message(air.nullifier_prefix, input.opening, commitment);
        Some(Self {
            input,
            commitment,
            owner_message,
            nullifier: Sha3_512::digest(nullifier_message).into(),
            nullifier_message,
        })
    }
}

impl SpendableNoteAir {
    fn check_roots(
        &self,
        input: SpendInput<'_>,
        commitment: [u8; 64],
        note_root: [u8; 64],
        eligibility_root: [u8; 64],
    ) -> Option<()> {
        if note_root == [0; 64] || self.membership.root(commitment, input.note_path) != note_root {
            return None;
        }
        self.eligibility
            .check_inputs(input.eligibility(), eligibility_root)
    }
}
