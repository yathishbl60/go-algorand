// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical private opening bytes only; no hash, lookup or authorization proof.
mod base;
mod constraints;
mod position;
mod program;
mod trace;
mod validation;
mod values;

use core::marker::PhantomData;
use p3_field::PrimeField64;

pub use position::RowPosition;
pub use program::ProgramColumn;

pub const WIDTH: usize = 11;
pub const PREPROCESSED: usize = 9;
pub const ENCODED_SIZE: usize = 512;
pub const BYTE_BITS: usize = 8;
pub const PREVIOUS: usize = 8;
pub const BIT_COUNT: usize = 9;
pub const END_INVERSE: usize = 10;
const CONTENT_SIZE: usize = 241;
const PREFIX_SIZE: usize = 73;
const NONZERO: [(usize, usize); 4] = [(73, 137), (137, 169), (177, 209), (209, 241)];

/// Constructor-owned scope and row program must match trusted preprocessing.
#[derive(Clone)]
pub struct NoteBytesAir<F: PrimeField64> {
    prefix: [u8; PREFIX_SIZE],
    count: usize,
    rows: usize,
    field: PhantomData<F>,
}

impl<F: PrimeField64> NoteBytesAir<F> {
    /// The field must injectively represent every uint16 and all bit counts.
    pub fn new(asset: u64, policy: [u8; 64], count: usize) -> Option<Self> {
        if asset == 0 || policy == [0; 64] || !(1..=4).contains(&count) || F::ORDER_U64 <= 65535 {
            return None;
        }
        let mut prefix = [0; PREFIX_SIZE];
        prefix[0] = 1;
        prefix[1..9].copy_from_slice(&asset.to_be_bytes());
        prefix[9..].copy_from_slice(&policy);
        Some(Self {
            prefix,
            count,
            rows: (count * ENCODED_SIZE).next_power_of_two(),
            field: PhantomData,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns a new fixed scope; rejects invalid or undersized padded heights.
    pub fn padded_to(&self, rows: usize) -> Option<Self> {
        let minimum = (self.count * ENCODED_SIZE).next_power_of_two();
        if !rows.is_power_of_two() || rows < minimum || rows > 65536 {
            return None;
        }
        let mut padded = self.clone();
        padded.rows = rows;
        Some(padded)
    }

    pub fn note_count(&self) -> usize {
        self.count
    }
}
