// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Four private ML-KEM NTT dot products with fixed quadratic-pair arithmetic.
//! Matrix/noise derivation and enclosing K-PKE bindings remain separate.
mod accumulation;
mod binding;
mod inputs;
mod layout;
mod relation;
mod schedule;
mod trace;
mod writer;
mod writer_step;

use crate::Value;
pub use inputs::{DotInputs, Polynomial};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const WIDTH: usize = 6161;
pub const ROWS: usize = 2048;
pub const SCHEDULE_WIDTH: usize = 145;
const PUBLIC: usize = 2304;
const RHAT: usize = 3072;
const OUTPUT: usize = 3840;
const CORE: usize = 4864;
const WORK: usize = CORE;
const STATE: usize = WORK + 512;
const A0: usize = WORK + 768;
const B0: usize = WORK + 818;
const GAMMA: usize = WORK + 868;
const PRODUCT: usize = WORK + 1143;
const ACC_INPUT: usize = WORK + 1195;
const ACC_OUTPUT: usize = ACC_INPUT + 50;
const CARRY: usize = ACC_OUTPUT + 50;
const OP: usize = 128;
const PRE_GAMMA: usize = 140;
const END: usize = 141;
const ACTIVE: usize = 1536;

#[derive(Clone, Copy)]
pub struct MlKemDotAir;

impl BaseAir<Value> for MlKemDotAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        SCHEDULE_WIDTH
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemDotAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_constants(builder);
        self.eval_polynomials(builder);
        self.eval_operands(builder);
        self.eval_arithmetic(builder);
        self.eval_accumulation(builder);
    }
}
