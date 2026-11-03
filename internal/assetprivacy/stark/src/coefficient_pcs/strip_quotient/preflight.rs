// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Config, Error, ProverData};
use super::{V, budget};
use crate::compact_storage::QuotientStrips;
use p3_batch_stark::quotient_scheduler::QuotientJob;

pub(super) fn validate(job: &QuotientJob<'_, Config>) -> Result<QuotientStrips<V>, Error> {
    job.pcs.limits.validate()?;
    if job.preprocessed.is_some() || job.layout.preprocessed_width != 0 {
        return Err(Error::Unsupported);
    }
    let strips =
        QuotientStrips::new(job.trace_domain, job.quotient_domain).ok_or(Error::Unsupported)?;
    let rows = job.trace_domain.size();
    let main = matrix(job.main, rows)?;
    if main != job.layout.main_width {
        return Err(Error::Shape);
    }
    let permutation = permutation_width(job, rows)?;
    budget::reserve(job, &strips, main, permutation)?;
    Ok(strips)
}

fn permutation_width(job: &QuotientJob<'_, Config>, rows: usize) -> Result<usize, Error> {
    job.permutation
        .map(|data| matrix(data, rows))
        .transpose()
        .map(|width| width.unwrap_or(0))
}

fn matrix((data, index): (&ProverData, usize), rows: usize) -> Result<usize, Error> {
    let source = data.inner.sources.get(index).ok_or(Error::Shape)?;
    if source.trace_rows() != rows || source.coefficient_rows() != rows * 2 {
        return Err(Error::Shape);
    }
    Ok(source.relation_width())
}
