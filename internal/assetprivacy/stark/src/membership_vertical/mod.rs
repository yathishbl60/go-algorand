// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Staged depth-32 membership with private leaf, path and intermediate digests.
mod binding;
mod boundary;
mod child;
mod components;
mod layout;
mod message;
mod schedule;
mod trace;
mod witness;
mod write;
mod writer;

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 4096;
pub const WIDTH: usize = 4950;
pub const PREPROCESSED: usize = 36;
pub(crate) const FRAME_ROWS: usize = 3105;
const HASH: usize = 4853;
const LEAF: usize = HASH;
const CURRENT: usize = LEAF + 32;
const DIGEST: usize = CURRENT + 32;
const DIR: usize = DIGEST + 32;
const FLAGS: usize = 32;
const STAGE_ROWS: usize = 97;
const ACTIVE_ROWS: usize = 32 * STAGE_ROWS;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/MerkleNode/";

const _: () = assert!(HASH == p3_keccak_air::NUM_KECCAK_COLS + 277 * 8 + 4);
const _: () = assert!(DIR + 1 == WIDTH);
const _: () = assert!(FRAME_ROWS == ACTIVE_ROWS + 1);

#[derive(Clone)]
pub struct MembershipVerticalAir {
    hash: Sha3Air,
    prefix: Vec<u8>,
}

impl MembershipVerticalAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        if network == [0; 32] || asset == 0 || policy == [0; 64] {
            return None;
        }
        let mut prefix = DOMAIN.to_vec();
        prefix.extend(network);
        prefix.extend(asset.to_be_bytes());
        prefix.extend(policy);
        let hash = Sha3Air::new(277)?;
        Some(Self { hash, prefix })
    }
}

impl BaseAir<Value> for MembershipVerticalAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        32
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build(ROWS))
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MembershipVerticalAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_fixed(builder);
        self.eval_boundaries(builder);
        self.eval_prefix(builder);
        self.eval_child(builder);
    }
}
