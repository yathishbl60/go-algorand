// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secret spending-key possession, bound to an exact native registration group.
mod absorption;
mod binding;
mod boundary;
mod public;
mod schedule;
mod trace;
mod trace_writer;

use crate::{Value, serial_keccak};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use public::PublicStatement;
pub use trace::GroupWitness;

pub const ROWS: usize = 512;
pub const PREPROCESSED: usize = serial_keccak::PREPROCESSED;
pub const PUBLIC_VALUES: usize = 64;
const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/SpendingKey/";
const PREFIX: usize = 77;
const MESSAGE: usize = PREFIX + 32;
const WORK: usize = 33;
pub const WIDTH: usize = WORK + serial_keccak::WIDTH;
const _: () = assert!(crate::MIN_TRACE_ROWS == ROWS);

#[derive(Clone)]
pub struct OwnerRegistrationAir {
    prefix: [u8; PREFIX],
}

impl OwnerRegistrationAir {
    pub fn new(network: [u8; 32]) -> Option<Self> {
        if network == [0; 32] {
            return None;
        }
        let mut prefix = [0; PREFIX];
        prefix[..DOMAIN.len()].copy_from_slice(DOMAIN);
        prefix[DOMAIN.len()..].copy_from_slice(&network);
        Some(Self { prefix })
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
        serial_keccak::eval(builder, WORK, 0);
        self.eval_initial(builder);
        self.eval_digest(builder);
        self.eval_absorption(builder);
        self.eval_secret(builder);
        self.eval_group(builder);
        self.eval_hold(builder);
    }
}
