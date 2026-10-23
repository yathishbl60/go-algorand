// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Binds a public nullifier to a privately owned note's exact committed serial.
//! Membership, nullifier freshness and transfer conservation remain separate.

mod binding;
mod components;
mod trace;
mod value;

use p3_air::{Air, AirBuilder, BaseAir};

use crate::{Value, owned_note_air::OwnedNoteAir, sha3_air::Sha3Air};

const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/Nullifier/";

#[derive(Clone)]
pub struct NullifiedNoteAir {
    owned: OwnedNoteAir,
    hash: Sha3Air,
    prefix: Vec<u8>,
}

impl NullifiedNoteAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        let owned = OwnedNoteAir::new(network, asset, policy)?;
        let mut prefix = DOMAIN.to_vec();
        prefix.extend(network);
        prefix.extend(asset.to_be_bytes());
        prefix.extend(policy);
        let hash = Sha3Air::new(prefix.len() + 32)?;
        Some(Self {
            owned,
            hash,
            prefix,
        })
    }
    fn digest_offset(&self) -> usize {
        self.owned.width() + self.hash.width()
    }
    pub(crate) fn width(&self) -> usize {
        self.digest_offset() + 32
    }
}

impl BaseAir<Value> for NullifiedNoteAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        32
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for NullifiedNoteAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_prefix(builder);
        self.eval_serial(builder);
        self.eval_digest(builder);
    }
}
