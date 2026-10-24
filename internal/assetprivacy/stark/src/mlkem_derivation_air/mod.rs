// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical private ML-KEM key hashing and encapsulation-seed derivation.
//! This does not yet constrain K-PKE, admission, KDF/AEAD or registry authority.
mod binding;
mod canonical;
mod components;
mod layout;
mod schedule;
mod trace;
mod witness;

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;

pub const ROWS: usize = 1024;
pub const KEY_BYTES: usize = 1184;
const COEFFICIENTS: usize = 768;

#[derive(Clone)]
pub struct MlKemDerivationAir {
    key_hash: Sha3Air,
    expand: Sha3Air,
}

impl MlKemDerivationAir {
    pub fn new() -> Option<Self> {
        Some(Self {
            key_hash: Sha3Air::new256(KEY_BYTES)?.with_trace_rows(ROWS)?,
            expand: Sha3Air::new(64)?.with_trace_rows(ROWS)?,
        })
    }
}

impl BaseAir<Value> for MlKemDerivationAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn preprocessed_width(&self) -> usize {
        COEFFICIENTS
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(self.schedule())
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemDerivationAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_hash_binding(builder);
        self.eval_output(builder);
        self.eval_canonical(builder);
    }
}
