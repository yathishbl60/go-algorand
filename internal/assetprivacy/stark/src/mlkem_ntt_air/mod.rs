// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private fixed-schedule ML-KEM NTT, not a complete KEM or native verifier.
mod binding;
mod layout;
mod operation;
mod program;
mod schedule;
mod trace;
mod transition;
mod view;
mod witness;
mod write;
mod writer;

use crate::Value;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const WIDTH: usize = 920;
pub const SCHEDULE_WIDTH: usize = 514;
const STATE: usize = 0;
const INPUT: usize = 256;
const OUTPUT: usize = 512;
const LEFT: usize = 768;
const RIGHT: usize = 793;
const PRODUCT: usize = 818;
const QUOTIENT: usize = 843;
const LOW: usize = 868;
const HIGH: usize = 893;
const CARRY: usize = 918;
const BORROW: usize = 919;
const ZETA: usize = 512;
const SCALE: usize = 513;

#[derive(Clone, Copy)]
pub struct MlKemNttAir {
    inverse: bool,
}

impl MlKemNttAir {
    pub const fn forward() -> Self {
        Self { inverse: false }
    }
    pub const fn inverse() -> Self {
        Self { inverse: true }
    }
    pub const fn trace_rows(&self) -> usize {
        if self.inverse { 2048 } else { 1024 }
    }
}

impl BaseAir<Value> for MlKemNttAir {
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

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemNttAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_bindings(builder);
        self.eval_operands(builder);
        self.eval_operation(builder);
        self.eval_transition(builder);
    }
}
