// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secret spending-key possession, bound to an exact native registration group.
mod binding;
mod public;
mod schedule;
mod trace;

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use public::PublicStatement;
pub use trace::GroupWitness;

pub const ROWS: usize = 512;
pub const PREPROCESSED: usize = 4;
pub const PUBLIC_VALUES: usize = 64;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/SpendingKey/";
const PREFIX: usize = 77;
const MESSAGE: usize = PREFIX + 32;
const WORK: usize = 33;
const HASH_WIDTH: usize = 3507;
pub const WIDTH: usize = WORK + HASH_WIDTH;
const _: () = assert!(p3_keccak_air::NUM_KECCAK_COLS + MESSAGE * 8 + 2 == HASH_WIDTH);
const _: () = assert!(crate::MIN_TRACE_ROWS == ROWS);

#[derive(Clone)]
pub struct OwnerRegistrationAir {
    hash: Sha3Air,
    prefix: [u8; PREFIX],
}

impl OwnerRegistrationAir {
    pub fn new(network: [u8; 32]) -> Option<Self> {
        if network == [0; 32] {
            return None;
        }
        let hash = Sha3Air::new(MESSAGE)?;
        let mut prefix = [0; PREFIX];
        prefix[..DOMAIN.len()].copy_from_slice(DOMAIN);
        prefix[DOMAIN.len()..].copy_from_slice(&network);
        Some(Self { hash, prefix })
    }
}

impl BaseAir<Value> for OwnerRegistrationAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for OwnerRegistrationAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_hash(builder);
        self.eval_prefix(builder);
        self.eval_secret(builder);
        self.eval_group(builder);
    }
}
