// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, ProveLimits};
use super::{Layout, Prepared};
use crate::Value;
use core::mem::size_of;
use p3_air::BaseAir;
use p3_matrix::dense::RowMajorMatrix;

/// Fixed local main-trace requirements; these sizes contain no witness data.
/// Call before a trace builder: checking an existing matrix cannot prevent its
/// earlier allocation. Preprocessing and all P3 prover temporaries are additional.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProveRequirements {
    pub rows: usize,
    pub width: usize,
    pub preprocessed_width: usize,
    pub public_values: usize,
    pub trace_bytes: usize,
}

impl<A: BaseAir<Value>> Prepared<A> {
    pub fn prover_requirements(&self, limits: ProveLimits) -> Result<ProveRequirements, Error> {
        ProveRequirements::from_layout(self.layout, limits)
    }
}

impl ProveRequirements {
    fn from_layout(layout: Layout, limits: ProveLimits) -> Result<Self, Error> {
        let shape = layout.shape();
        let rows = layout.rows();
        let trace_bytes = checked_trace_bytes(rows, shape.width, limits.trace_bytes)?;
        Ok(Self {
            rows,
            width: shape.width,
            preprocessed_width: shape.preprocessed,
            public_values: shape.public,
            trace_bytes,
        })
    }

    pub(crate) fn check_trace(&self, trace: &RowMajorMatrix<Value>) -> Result<(), Error> {
        let elements = self.rows.checked_mul(self.width).ok_or(Error::TraceShape)?;
        if self.rows == 0
            || self.width == 0
            || trace.width != self.width
            || trace.values.len() != elements
        {
            return Err(Error::TraceShape);
        }
        Ok(())
    }
}

fn checked_trace_bytes(rows: usize, width: usize, limit: usize) -> Result<usize, Error> {
    if rows == 0 || width == 0 {
        return Err(Error::TraceShape);
    }
    let bytes = rows
        .checked_mul(width)
        .and_then(|elements| elements.checked_mul(size_of::<Value>()))
        .ok_or(Error::TraceLimit)?;
    if bytes > limit {
        return Err(Error::TraceLimit);
    }
    Ok(bytes)
}
