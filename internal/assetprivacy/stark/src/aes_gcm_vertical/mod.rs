// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed vertical AES blocks for the candidate GCM envelope, not full encryption.
mod binding;
mod cipher;
mod key;
mod layout;
mod output;
mod schedule;
mod trace;
mod view;
mod witness;
mod write;

use crate::Value;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const WIDTH: usize = 11216;
pub const ROWS: usize = 512;
pub const PUBLIC_VALUES: usize = 268;
const SCHEDULE: usize = 61;
const KEY: usize = 0;
const BOXES: usize = 1920;
const MIX: usize = 5776;
const STATE: usize = 6384;
const OUT: usize = 6512;
const INPUT: usize = 6640;
const PT: usize = 6768;
const NONCE: usize = 10864;
const H: usize = 10960;
const MASK: usize = 11088;

#[derive(Clone, Copy)]
pub struct AesGcmAir;

impl BaseAir<Value> for AesGcmAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        SCHEDULE
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for AesGcmAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_bindings(builder);
        self.eval_key(builder);
        self.eval_cipher(builder);
        self.eval_outputs(builder);
    }
}
