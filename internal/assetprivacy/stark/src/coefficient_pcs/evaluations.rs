// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Dense, Domain, Error, ProverData, V};
use crate::compact_storage::MaskedTrace;
use p3_dft::Radix2Dit;

pub(super) fn evaluate(
    pcs: &CoefficientPcs,
    data: &ProverData,
    index: usize,
    domain: Domain,
    random: bool,
) -> Result<Dense, Error> {
    let source = data.inner.sources.get(index).ok_or(Error::Shape)?;
    let columns = if random {
        source.width()
    } else {
        source.relation_width()
    };
    bounded(source, columns, domain, pcs.limits.max_evaluation_values)
}

fn bounded(
    source: &MaskedTrace<V>,
    columns: usize,
    domain: Domain,
    limit: usize,
) -> Result<Dense, Error> {
    let values = domain.size().checked_mul(columns).ok_or(Error::Limits)?;
    if values > limit {
        return Err(Error::Limits);
    }
    source
        .evaluate_columns(0..columns, domain, limit, &Radix2Dit::default())
        .ok_or(Error::Unsupported)
}
