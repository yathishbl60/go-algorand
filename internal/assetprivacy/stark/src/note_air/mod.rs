// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate note-opening constraints. Membership and spending authority are separate.

mod constraints;
mod trace;
mod value;
mod writer;

use p3_air::{Air, AirBuilder, BaseAir};

use crate::{Value, prefix_builder::PrefixBuilder, sha3_air::Sha3Air};

const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/Commitment/";
const OPENING_START: usize = DOMAIN.len() + 32;
const ENCODED_SIZE: usize = 512;
const CONTENT_SIZE: usize = 241;
const NONZERO: [(usize, usize); 4] = [(73, 137), (137, 169), (177, 209), (209, 241)];

/// Network, asset and policy come from the verifier's bound public statement.
#[derive(Clone)]
pub struct NoteCommitmentAir {
    hash: Sha3Air,
    prefix: Vec<u8>,
}

impl NoteCommitmentAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        if network == [0; 32] || asset == 0 || policy == [0; 64] {
            return None;
        }
        let mut prefix = DOMAIN.to_vec();
        prefix.extend(network);
        prefix.push(1);
        prefix.extend(asset.to_be_bytes());
        prefix.extend(policy);
        let hash = Sha3Air::new(OPENING_START + ENCODED_SIZE)?;
        Some(Self { hash, prefix })
    }
    pub(crate) fn width(&self) -> usize {
        self.hash.width() + NONZERO.len()
    }
}

impl BaseAir<Value> for NoteCommitmentAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        32
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for NoteCommitmentAir {
    fn eval(&self, builder: &mut AB) {
        self.hash.eval(&mut PrefixBuilder {
            inner: builder,
            width: self.hash.width(),
        });
        self.eval_prefix(builder);
        self.eval_padding(builder);
        self.eval_nonzero(builder);
    }
}
