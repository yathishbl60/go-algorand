// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical input openings and private spending-key ownership only.
//! Membership, nullifiers and complete native spending remain outside this AIR.
mod base;
mod binding;
mod eval;
mod factory;
mod ports;
mod program;
mod public;
mod schema;
mod secret;
mod trace;
mod witness;

use crate::{compact_keccak_air::CompactKeccakAir, note_bytes_air::NoteBytesAir};
use p3_field::PrimeField64;
pub(crate) use schema::*;

#[derive(Clone)]
pub struct OwnedNotesAir<F: PrimeField64> {
    pub(crate) note: NoteBytesAir<F>,
    pub(crate) hash: CompactKeccakAir<F>,
    pub(crate) network: [u8; 32],
    pub(crate) group: [u8; 64],
    pub(crate) rows: usize,
    pub(crate) count: usize,
    pub(crate) input_base: [u64; 25],
    pub(crate) periodic: Vec<Vec<F>>,
    pub(crate) public_values: Vec<F>,
}

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub fn new(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        group: [u8; 64],
        count: usize,
        rows: usize,
    ) -> Option<Self> {
        if network == [0; 32] || group == [0; 64] || !(1..=2).contains(&count) {
            return None;
        }
        Self::construct((network, asset, policy, group, count, rows))
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn note_count(&self) -> usize {
        self.count
    }

    pub fn public_values(&self) -> &[F] {
        &self.public_values
    }
}
