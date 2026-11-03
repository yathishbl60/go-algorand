// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Dense, Error, ProverData, V, budget, commit, preflight};
use crate::compact::LOG_BLOWUP;
use crate::compact_storage::{MaskedTrace, TreeCap};
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::{Matrix, bitrev::BitReversibleMatrix};
use std::sync::Arc;

/// Only freshly masked quotient LDEs may be imported; degree checks do not prove entropy.
pub(crate) fn ldes(pcs: &CoefficientPcs, ldes: Vec<Dense>) -> Result<(TreeCap, ProverData), Error> {
    let rows = validate(pcs, &ldes)?;
    let mut sources = Vec::new();
    sources
        .try_reserve_exact(ldes.len())
        .map_err(|_| Error::Allocation)?;
    for lde in ldes {
        sources.push(Arc::new(interpolate(lde, rows)?));
    }
    commit::finish(pcs, sources, preflight::canonical_domain(rows)?)
}

fn validate(pcs: &CoefficientPcs, ldes: &[Dense]) -> Result<usize, Error> {
    pcs.limits.validate()?;
    if !(1..=16).contains(&ldes.len()) {
        return Err(Error::Shape);
    }
    let mut total = (0usize, 0usize);
    let mut rows = None;
    evaluation_budget(pcs, ldes)?;
    for lde in ldes {
        let height = shape(lde)?;
        if rows.is_some_and(|previous| previous != height) {
            return Err(Error::Unsupported);
        }
        rows = Some(height);
        preflight::masked_shape(height, lde.width(), pcs.limits, &mut total)?;
    }
    budget::reserve(total.0, total.1, pcs.limits)?;
    rows.ok_or(Error::Shape)
}

fn evaluation_budget(pcs: &CoefficientPcs, ldes: &[Dense]) -> Result<(), Error> {
    let mut cells = 0usize;
    for lde in ldes {
        cells = cells.checked_add(lde.values.len()).ok_or(Error::Limits)?;
    }
    if cells > pcs.limits.max_evaluation_values {
        return Err(Error::Limits);
    }
    Ok(())
}

fn shape(lde: &Dense) -> Result<usize, Error> {
    let width = lde.width();
    if width == 0 || !lde.values.len().is_multiple_of(width) {
        return Err(Error::Shape);
    }
    let rows = lde.height();
    if !rows.is_power_of_two() {
        return Err(Error::Shape);
    }
    rows.checked_shr(LOG_BLOWUP as u32).ok_or(Error::Shape)
}

fn interpolate(lde: Dense, coefficient_rows: usize) -> Result<MaskedTrace<V>, Error> {
    let natural = lde.bit_reverse_rows().to_row_major_matrix();
    let coefficients = Radix2Dit::default().coset_idft_batch(natural, V::GENERATOR);
    let end = coefficient_rows
        .checked_mul(coefficients.width())
        .ok_or(Error::Limits)?;
    if coefficients.values[end..]
        .iter()
        .any(|value| *value != V::ZERO)
    {
        return Err(Error::Shape);
    }
    MaskedTrace::from_masked_coefficients(compact(coefficients, end)?).ok_or(Error::Shape)
}

// Drop the full-LDE allocation instead of retaining its eightfold spare capacity.
fn compact(coefficients: Dense, end: usize) -> Result<Dense, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(end)
        .map_err(|_| Error::Allocation)?;
    values.extend_from_slice(&coefficients.values[..end]);
    Ok(Dense::new(values, coefficients.width()))
}
