// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Dense, Domain, E, Error, ProverData, V, budget, commit, preflight};
use crate::compact::RANDOM_CODEWORDS;
use crate::compact_storage::{MaskedTrace, TreeCap};
use p3_field::BasedVectorSpace;
use rand::{RngExt, rngs::StdRng};
use std::sync::Arc;

const WIDTH: usize = <E as BasedVectorSpace<V>>::DIMENSION + RANDOM_CODEWORDS;

/// A uniform coefficient vector has the same distribution as stock uniform evaluations.
pub(crate) fn commit(
    pcs: &CoefficientPcs,
    domains: Vec<Domain>,
) -> Result<(TreeCap, ProverData), Error> {
    let rows = validate(pcs, &domains)?;
    let mut sources = Vec::new();
    sources
        .try_reserve_exact(domains.len())
        .map_err(|_| Error::Allocation)?;
    let mut random = pcs.fork_masks()?;
    for _ in domains {
        sources.push(Arc::new(uniform(rows, &mut random)?));
    }
    commit::finish(pcs, sources, preflight::canonical_domain(rows)?)
}

fn validate(pcs: &CoefficientPcs, domains: &[Domain]) -> Result<usize, Error> {
    pcs.limits.validate()?;
    if !(1..=16).contains(&domains.len()) {
        return Err(Error::Shape);
    }
    let mut total = (0usize, 0usize);
    let mut rows = None;
    for domain in domains {
        let height = domain.size();
        preflight::original_domain(*domain, height / 2)?;
        if rows.is_some_and(|previous| previous != height) {
            return Err(Error::Unsupported);
        }
        rows = Some(height);
        preflight::masked_shape(height, WIDTH, pcs.limits, &mut total)?;
    }
    budget::reserve(total.0, total.1, pcs.limits)?;
    rows.ok_or(Error::Shape)
}

fn uniform(rows: usize, random: &mut StdRng) -> Result<MaskedTrace<V>, Error> {
    let cells = rows.checked_mul(WIDTH).ok_or(Error::Limits)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(cells)
        .map_err(|_| Error::Allocation)?;
    values.extend((0..cells).map(|_| random.random::<V>()));
    MaskedTrace::from_masked_coefficients(Dense::new(values, WIDTH)).ok_or(Error::Shape)
}
