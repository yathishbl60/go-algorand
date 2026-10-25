// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private registered output-owner commitment; spending authority is separate.
mod binding;
mod layout;
mod message;
mod prepared;
mod trace;
mod writer;

use crate::{Value, sha3_air::Sha3Air};
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};

pub const ROWS: usize = 512;
pub const WIDTH: usize = 4816;
const HASH_WIDTH: usize = 4781;
const COMMIT: usize = HASH_WIDTH;
const NONZERO: usize = COMMIT + 32;
const MESSAGE_BYTES: usize = 268;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/KeyAuthorization/";
const ACCOUNT: usize = 155;
const ROLE: usize = 187;
const EPOCH: usize = 188;
const CREATED: usize = 196;
const OWNER: usize = 204;

#[derive(Clone)]
pub struct SpendKeyAir {
    hash: Sha3Air,
    prefix: [u8; 155],
}

impl SpendKeyAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        if network == [0; 32] || asset == 0 || policy == [0; 64] {
            return None;
        }
        let hash = Sha3Air::new(MESSAGE_BYTES)?;
        (hash.width() == HASH_WIDTH && hash.trace_rows() == ROWS).then(|| Self {
            hash,
            prefix: message::prefix(network, asset, policy),
        })
    }
}

impl BaseAir<Value> for SpendKeyAir {
    fn width(&self) -> usize {
        WIDTH
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for SpendKeyAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_prefix(builder);
        self.eval_digest_carry(builder);
        self.eval_nonzero(builder);
    }
}
