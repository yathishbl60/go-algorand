// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete private ML-KEM NTT-domain product under trusted fixed preprocessing.
//! Parent KEM still must bind operands, sampling, encoding and ciphertexts.
mod binding;
mod layout;
mod relation;
mod schedule;
mod trace;
mod view;
mod witness;
mod write;

use crate::Value;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub(crate) use view::canonical;
pub(crate) use witness::Pair;
pub(crate) use write::{canonical as write_canonical, pair as write_pair};

pub const WIDTH: usize = 1195;
pub const ROWS: usize = 512;
const SCHEDULE: usize = 129;
const LEFT: usize = 0;
const RIGHT: usize = 256;
const OUTPUT: usize = 512;
const A0: usize = 768;
const A1: usize = 793;
const B0: usize = 818;
const B1: usize = 843;
const GAMMA: usize = 868;
const P11: usize = 893;
const GP11: usize = 943;
const P00: usize = 993;
const P01: usize = 1043;
const P10: usize = 1093;
const C0: usize = 1143;
const C1: usize = 1168;
const CARRY0: usize = 1193;
const CARRY1: usize = 1194;

#[derive(Clone, Copy)]
pub struct MlKemProductAir;

impl BaseAir<Value> for MlKemProductAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn preprocessed_width(&self) -> usize {
        SCHEDULE
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemProductAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_bindings(builder);
        self.eval_arithmetic(builder);
    }
}
