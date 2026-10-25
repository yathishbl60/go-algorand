// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private canonical effective-unfrozen snapshot; authority is supplied by native state.
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
const MESSAGE_BYTES: usize = 199;
const HASH_WIDTH: usize = p3_keccak_air::NUM_KECCAK_COLS + MESSAGE_BYTES * 8 + 3;
const COMMIT: usize = HASH_WIDTH;
const NONZERO: usize = COMMIT + 32;
pub const WIDTH: usize = NONZERO + 2;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/Eligibility/";
const ACCOUNT: usize = 150;
const REVISION: usize = 182;
const UPDATED: usize = 190;
const FROZEN: usize = 198;

#[derive(Clone)]
pub struct EligibleRecordAir {
    hash: Sha3Air,
    prefix: [u8; ACCOUNT],
}

impl EligibleRecordAir {
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

impl BaseAir<Value> for EligibleRecordAir {
    fn width(&self) -> usize {
        WIDTH
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for EligibleRecordAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_prefix(builder);
        self.eval_digest_carry(builder);
        self.eval_nonzero(builder);
    }
}
