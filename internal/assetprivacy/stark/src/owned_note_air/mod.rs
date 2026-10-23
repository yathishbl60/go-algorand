// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private input-note ownership component; not a complete spending relation.
//! The input commitment and owner key remain private trace columns.

mod binding;
mod components;
mod trace;
mod value;

use p3_air::{Air, AirBuilder, BaseAir};

use crate::{Value, note_air::NoteCommitmentAir, sha3_air::Sha3Air};

const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/SpendingKey/";

#[derive(Clone)]
pub struct OwnedNoteAir {
    note: NoteCommitmentAir,
    owner: Sha3Air,
    prefix: Vec<u8>,
}

impl OwnedNoteAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        let note = NoteCommitmentAir::new(network, asset, policy)?;
        let mut prefix = DOMAIN.to_vec();
        prefix.extend(network);
        let owner = Sha3Air::new(prefix.len() + 32)?;
        Some(Self {
            note,
            owner,
            prefix,
        })
    }
    fn digest_offset(&self) -> usize {
        self.note.width() + self.owner.width()
    }
    pub(crate) fn width(&self) -> usize {
        self.digest_offset() + 65
    }
}

impl BaseAir<Value> for OwnedNoteAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        0
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for OwnedNoteAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_owner_prefix(builder);
        self.eval_owner_digest(builder);
        self.eval_secret(builder);
        self.eval_commitment(builder);
    }
}
