// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Config;
use super::{E, Error, V};
use crate::compact_storage::QuotientStrips;
use p3_batch_stark::quotient_scheduler::QuotientJob;
use p3_field::BasedVectorSpace;

const EXTENSION: usize = <E as BasedVectorSpace<V>>::DIMENSION;
const SELECTORS: usize = 4;
// Original declarations, natural evaluations and packed evaluations coexist.
const PERIODIC_COPIES: usize = 3;

pub(super) fn reserve(
    job: &QuotientJob<'_, Config>,
    strips: &QuotientStrips<V>,
    main: usize,
    permutation: usize,
) -> Result<(), Error> {
    let width = width(main, permutation, job.layout.num_periodic_columns)?;
    let values = values(strips, width)?;
    if values > job.pcs.limits.max_evaluation_values {
        return Err(Error::Limits);
    }
    Ok(())
}

fn values(strips: &QuotientStrips<V>, width: usize) -> Result<usize, Error> {
    let active = strips
        .strip_rows()
        .checked_mul(width)
        .ok_or(Error::Limits)?;
    let output = strips
        .total_rows()
        .checked_mul(EXTENSION)
        .ok_or(Error::Limits)?;
    active.checked_add(output).ok_or(Error::Limits)
}

fn width(main: usize, permutation: usize, periodic: usize) -> Result<usize, Error> {
    let periodic = periodic.checked_mul(PERIODIC_COPIES).ok_or(Error::Limits)?;
    main.checked_add(permutation)
        .and_then(|n| n.checked_add(periodic))
        .and_then(|n| n.checked_add(SELECTORS + EXTENSION))
        .ok_or(Error::Limits)
}
