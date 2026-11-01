// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error};
use crate::Value;
use p3_matrix::dense::RowMajorMatrix;

/// Local trace and proof limits, not approved consensus or deployment budgets.
/// The trace ceiling excludes preprocessing, hiding expansion and P3 temporaries.
#[derive(Clone, Copy)]
pub struct ProveLimits {
    pub(super) trace_bytes: usize,
    pub(super) proof: DecodeLimits,
}

impl ProveLimits {
    pub fn new(trace_bytes: usize, proof: DecodeLimits) -> Result<Self, Error> {
        if trace_bytes == 0 {
            return Err(Error::TraceLimit);
        }
        Ok(Self { trace_bytes, proof })
    }
}

impl ProveLimits {
    pub(crate) fn check_wire_floor(self, minimum: usize) -> Result<(), Error> {
        if minimum > self.proof.wire.min(self.proof.allocation) {
            return Err(Error::ProofBudget);
        }
        Ok(())
    }

    pub(crate) fn check_trace_capacity(self, trace: &RowMajorMatrix<Value>) -> Result<(), Error> {
        let bytes = trace
            .values
            .capacity()
            .checked_mul(core::mem::size_of::<Value>())
            .ok_or(Error::TraceLimit)?;
        if bytes > self.trace_bytes {
            return Err(Error::TraceLimit);
        }
        Ok(())
    }
}
