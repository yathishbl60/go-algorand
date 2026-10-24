// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private K-PKE polynomials to public ML-KEM-768 ciphertext compression.
//! This component does not derive the polynomials or authorize encapsulation.
mod binding;
mod layout;
mod operation;
mod packing;
mod schedule;
mod trace;
mod view;
mod witness;
mod writer;

use crate::Value;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub use trace::CiphertextWitness;

pub const ROWS: usize = 2048;
pub const WIDTH: usize = 1119;
pub const PUBLIC_VALUES: usize = 544;
pub const COEFFICIENTS: usize = 1024;
const SCRATCH: usize = 1024;
const LOCAL_BITS: usize = 1049;
const ROLLOVER: usize = 1061;
const REMAINDER: usize = 1062;
const WORD_BITS: usize = 1087;

#[derive(Clone, Copy)]
pub struct MlKemCiphertextAir;

impl BaseAir<Value> for MlKemCiphertextAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        COEFFICIENTS
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemCiphertextAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_input(builder);
        self.eval_operation(builder);
        self.eval_words(builder);
        self.eval_packing(builder);
    }
}

pub(super) const fn bit_position(index: usize) -> usize {
    if index < 768 {
        index * 10
    } else {
        7680 + (index - 768) * 4
    }
}
