// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete fixed-depth private SHA3-512 membership relation.
//! Horizontal composition is costly; no proving resource claim is established.

mod binding;
mod components;
mod layout;
mod message;
mod path;
mod trace;
mod trace_join;

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};

pub const DEPTH: usize = 32;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/MerkleNode/";

#[derive(Clone)]
pub struct MembershipAir {
    hash: Sha3Air,
    prefix: Vec<u8>,
}

/// Private witness; siblings are ordered from the leaf to the root.
pub struct MembershipPath {
    pub index: u32,
    pub siblings: [[u8; 64]; DEPTH],
}

impl MembershipAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        if network == [0; 32] || asset == 0 || policy == [0; 64] {
            return None;
        }
        let mut prefix = DOMAIN.to_vec();
        prefix.extend(network);
        prefix.extend(asset.to_be_bytes());
        prefix.extend(policy);
        let hash = Sha3Air::new(prefix.len() + 1 + 128)?;
        Some(Self { hash, prefix })
    }
}

impl BaseAir<Value> for MembershipAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        32
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MembershipAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        for level in 0..DEPTH {
            self.eval_prefix(builder, level);
            self.eval_stage(builder, level);
        }
        self.eval_root(builder);
    }
}
