// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, Proof};
use crate::Value;
use p3_air::BaseAir;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) struct Shape {
    pub width: usize,
    pub preprocessed: usize,
    pub public: usize,
    pub main_next: bool,
    pub preprocessed_next: bool,
}

impl Shape {
    pub(super) fn from_air<A: BaseAir<Value>>(air: &A) -> Result<Self, Error> {
        let shape = Self {
            width: air.width(),
            preprocessed: air.preprocessed_width(),
            public: air.num_public_values(),
            main_next: !air.main_next_row_columns().is_empty(),
            preprocessed_next: !air.preprocessed_next_row_columns().is_empty(),
        };
        if !air.public_boundary_io().is_empty()
            || shape.width == 0
            || shape.width > 32768
            || shape.preprocessed == 0
            || shape.preprocessed > 4096
            || shape.public > 65536
        {
            return Err(Error::Shape);
        }
        Ok(shape)
    }

    pub(super) fn check(&self, proof: &Proof) -> Result<(), Error> {
        let values = &proof.opened_values;
        if values.trace_local.len() != self.width
            || values.trace_next.as_ref().map(Vec::len) != self.main_next.then_some(self.width)
            || values.quotient_chunks.len() > 128
            || values.quotient_chunks.iter().any(|v| v.len() != 5)
        {
            return Err(Error::Shape);
        }
        self.check_preprocessed(proof)
    }

    fn check_preprocessed(&self, proof: &Proof) -> Result<(), Error> {
        let values = &proof.opened_values;
        let pre = values.preprocessed.as_ref().ok_or(Error::Shape)?;
        if pre.local.len() != self.preprocessed
            || pre.next.as_ref().map(Vec::len)
                != self.preprocessed_next.then_some(self.preprocessed)
            || values.random.as_ref().map(Vec::len) != Some(5)
        {
            return Err(Error::Shape);
        }
        Ok(())
    }
}
