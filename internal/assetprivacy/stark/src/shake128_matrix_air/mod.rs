// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private SHAKE128 prefix of rho32||first||second for ML-KEM-768 matrices.
//! Rejection sampling and complete KEM/note/key bindings remain separate.
mod binding;
mod boundary;
mod layout;
mod schedule;
mod squeeze;
mod trace;
mod witness;

use crate::{Value, prefix_builder::PrefixBuilder};
pub use layout::Wires;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::{KeccakAir, NUM_KECCAK_COLS};
use p3_matrix::dense::RowMajorMatrix;

pub const RATE_BYTES: usize = 168;
pub const MAX_RATE_BLOCKS: usize = 16;
pub const ROWS: usize = 512;
const SEED: usize = NUM_KECCAK_COLS;
const OUTPUT: usize = SEED + 256;

#[derive(Clone, Copy)]
pub struct Shake128MatrixAir {
    first: u8,
    second: u8,
    blocks: usize,
    rows: usize,
}

impl Shake128MatrixAir {
    pub const fn new(first: u8, second: u8, blocks: usize) -> Option<Self> {
        if first >= 3 || second >= 3 || blocks == 0 || blocks > MAX_RATE_BLOCKS {
            return None;
        }
        Some(Self {
            first,
            second,
            blocks,
            rows: ROWS,
        })
    }
    pub(crate) fn with_rows(mut self, rows: usize) -> Option<Self> {
        if !matches!(rows, 512 | 1024 | 2048) {
            return None;
        }
        self.rows = rows;
        Some(self)
    }
    pub const fn trace_rows(&self) -> usize {
        self.rows
    }
    pub const fn output_bytes(&self) -> usize {
        self.blocks * RATE_BYTES
    }
    pub const fn trace_width(&self) -> usize {
        OUTPUT + self.output_bytes() * 8
    }
}

impl BaseAir<Value> for Shake128MatrixAir {
    fn width(&self) -> usize {
        self.trace_width()
    }
    fn preprocessed_width(&self) -> usize {
        self.blocks
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for Shake128MatrixAir {
    fn eval(&self, builder: &mut AB) {
        KeccakAir {}.eval(&mut PrefixBuilder {
            inner: builder,
            width: NUM_KECCAK_COLS,
        });
        self.eval_bindings(builder);
        self.eval_initial(builder);
        self.eval_squeeze(builder);
    }
}
