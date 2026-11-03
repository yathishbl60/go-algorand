// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Domain, Error, Limits, budget};
use crate::compact::{LOG_BLOWUP, RANDOM_CODEWORDS};
use p3_matrix::Matrix;

#[path = "preflight_shape.rs"]
mod shape;
pub(crate) use shape::{canonical_domain, original_domain};

pub(crate) fn regular(evaluations: &[(Domain, Dense)], limits: Limits) -> Result<Domain, Error> {
    limits.validate()?;
    if !(1..=16).contains(&evaluations.len()) {
        return Err(Error::Shape);
    }
    let mut total = (0usize, 0usize);
    let mut height = None;
    for (domain, trace) in evaluations {
        let rows = shape::trace_rows(*domain, trace)?;
        if height.is_some_and(|previous| previous != rows) {
            return Err(Error::Unsupported);
        }
        height = Some(rows);
        accumulate(rows, trace.width(), limits, &mut total)?;
    }
    budget::reserve(total.0, total.1, limits)?;
    canonical_domain(height.ok_or(Error::Shape)? * 2)
}

fn accumulate(
    rows: usize,
    width: usize,
    limits: Limits,
    total: &mut (usize, usize),
) -> Result<(), Error> {
    let columns = width.checked_add(RANDOM_CODEWORDS).ok_or(Error::Limits)?;
    let coefficient_rows = rows.checked_mul(2).ok_or(Error::Limits)?;
    let cells = coefficient_rows.checked_mul(columns).ok_or(Error::Limits)?;
    total.0 = total.0.checked_add(cells).ok_or(Error::Limits)?;
    total.1 = total.1.checked_add(columns).ok_or(Error::Limits)?;
    tile(coefficient_rows, columns, limits)
}

fn tile(coefficient_rows: usize, columns: usize, limits: Limits) -> Result<(), Error> {
    let lde = coefficient_rows
        .checked_shl(LOG_BLOWUP as u32)
        .ok_or(Error::Limits)?;
    let tile = lde
        .checked_mul(columns.min(limits.tile_columns))
        .ok_or(Error::Limits)?;
    if tile > limits.max_tile_values {
        return Err(Error::Limits);
    }
    Ok(())
}

pub(crate) fn masked_shape(
    coefficient_rows: usize,
    columns: usize,
    limits: Limits,
    total: &mut (usize, usize),
) -> Result<(), Error> {
    if !coefficient_rows.is_power_of_two()
        || !(1024..=131072).contains(&coefficient_rows)
        || columns <= RANDOM_CODEWORDS
    {
        return Err(Error::Shape);
    }
    budget::hiding(coefficient_rows / 2, 1)?;
    accumulate(
        coefficient_rows / 2,
        columns - RANDOM_CODEWORDS,
        limits,
        total,
    )
}
