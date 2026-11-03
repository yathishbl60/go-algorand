// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Dense, Domain, Error, ProverData, V, preflight};
use crate::compact_storage::{MaskedTrace, TreeCap};
use p3_dft::Radix2Dit;
use p3_matrix::Matrix;
use std::sync::Arc;

pub(crate) fn regular(
    pcs: &CoefficientPcs,
    evaluations: Vec<(Domain, Dense)>,
) -> Result<(TreeCap, ProverData), Error> {
    let domain = preflight::regular(&evaluations, pcs.limits)?;
    let mut sources = Vec::new();
    sources
        .try_reserve_exact(evaluations.len())
        .map_err(|_| Error::Allocation)?;
    let mut random = pcs.fork_masks()?;
    for (extended, trace) in evaluations {
        let original = preflight::original_domain(extended, trace.height())?;
        let masked = MaskedTrace::from_trace(trace, original, &Radix2Dit::default(), &mut random)
            .ok_or(Error::Allocation)?;
        sources.push(Arc::new(masked));
    }
    finish(pcs, sources, domain)
}

pub(crate) fn finish(
    pcs: &CoefficientPcs,
    sources: Vec<Arc<MaskedTrace<V>>>,
    domain: Domain,
) -> Result<(TreeCap, ProverData), Error> {
    let (cap, input) =
        pcs.input
            .commit_coefficients(sources.clone(), domain, pcs.limits.input())?;
    Ok((cap, ProverData::new(input, sources, domain)?))
}
