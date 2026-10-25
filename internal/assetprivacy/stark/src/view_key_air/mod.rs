// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private viewing-key record commitment; current authorization is separate.
mod binding;
mod canonical;
mod layout;
mod message;
mod prepared;
mod schedule;
mod trace;
mod writer;

use crate::{Value, sha3_air::Sha3Air};
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 1024;
pub const WIDTH: usize = 13816;
pub const PREPROCESSED: usize = 768;
const HASH_WIDTH: usize = 13757;
const COMMIT: usize = HASH_WIDTH;
const CANONICAL: usize = COMMIT + 32;
const NONZERO: usize = CANONICAL + 25;
const MESSAGE_BYTES: usize = 1388;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/KeyAuthorization/";
const ACCOUNT: usize = 155;
const ROLE: usize = 187;
const EPOCH: usize = 188;
const CREATED: usize = 196;
const KEY: usize = 204;

#[derive(Clone)]
pub struct ViewKeyAir {
    hash: Sha3Air,
    prefix: [u8; 155],
    role: u8,
}

impl ViewKeyAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64], role: u8) -> Option<Self> {
        if network == [0; 32] || asset == 0 || policy == [0; 64] || !matches!(role, 1 | 3 | 4) {
            return None;
        }
        let hash = Sha3Air::new(MESSAGE_BYTES)?.with_trace_rows(ROWS)?;
        (hash.width() == HASH_WIDTH).then(|| Self {
            hash,
            prefix: message::prefix(network, asset, policy),
            role,
        })
    }
}

impl BaseAir<Value> for ViewKeyAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for ViewKeyAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_prefix(builder);
        self.eval_digest_carry(builder);
        self.eval_nonzero(builder);
        self.eval_canonical(builder);
    }
}
