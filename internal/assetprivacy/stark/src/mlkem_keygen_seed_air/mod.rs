// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private FIPS203 K-PKE key-generation seed expansion for ML-KEM-768.
//! This component alone proves neither a public key nor registration possession.
mod binding;
mod layout;
mod trace;
pub(crate) mod writer;

use crate::{Value, sha3_air::Sha3Air};
pub use layout::{WIDTH, Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
pub const ROWS: usize = 512;
const _: () = assert!(crate::MIN_TRACE_ROWS == ROWS);
const HASH_WIDTH: usize = 2898;
const DIGEST: usize = HASH_WIDTH;
const RHO: usize = DIGEST + 32;
const SIGMA: usize = RHO + 256;
const _: () = assert!(p3_keccak_air::NUM_KECCAK_COLS + 33 * 8 + 1 == HASH_WIDTH);

#[derive(Clone)]
pub struct MlKemKeygenSeedAir {
    hash: Sha3Air,
}

impl MlKemKeygenSeedAir {
    pub fn new() -> Option<Self> {
        Some(Self {
            hash: Sha3Air::new(33)?,
        })
    }
}

impl BaseAir<Value> for MlKemKeygenSeedAir {
    fn width(&self) -> usize {
        WIDTH
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemKeygenSeedAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_dimension(builder);
        self.eval_output(builder);
    }
}
