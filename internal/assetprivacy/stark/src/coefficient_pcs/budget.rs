// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, Limits};
use crate::compact::{Challenge32, NUM_QUERIES, RANDOM_CODEWORDS, Value32};
use p3_field::BasedVectorSpace;

const EXTENSION: usize = <Challenge32 as BasedVectorSpace<Value32>>::DIMENSION;

pub(crate) fn hiding(mask_rows: usize, points: usize) -> Result<(), Error> {
    let required = EXTENSION
        .checked_mul(points.max(1))
        .and_then(|n| n.checked_add(NUM_QUERIES))
        .and_then(|n| n.checked_mul(2))
        .ok_or(Error::Limits)?;
    if RANDOM_CODEWORDS < EXTENSION || mask_rows < required {
        return Err(Error::HidingBudget);
    }
    Ok(())
}

pub(crate) fn reserve(
    coefficient_cells: usize,
    widths: usize,
    limits: Limits,
) -> Result<(), Error> {
    let queries = widths.checked_mul(NUM_QUERIES).ok_or(Error::Limits)?;
    let claims = widths.checked_mul(EXTENSION).ok_or(Error::Limits)?;
    if coefficient_cells > limits.max_coefficient_values
        || queries > limits.max_query_values
        || claims > limits.max_claim_values
    {
        return Err(Error::Limits);
    }
    Ok(())
}
