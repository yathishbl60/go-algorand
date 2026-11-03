// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Domain, Error, budget};
use crate::coefficient_pcs::V;
use crate::compact::LOG_BLOWUP;
use p3_field::{Field, PrimeCharacteristicRing, TwoAdicField};
use p3_matrix::Matrix;

pub(super) fn trace_rows(domain: Domain, trace: &Dense) -> Result<usize, Error> {
    let width = trace.width();
    if width == 0 || !trace.values.len().is_multiple_of(width) {
        return Err(Error::Shape);
    }
    let rows = trace.height();
    if !(512..=65536).contains(&rows) || !rows.is_power_of_two() {
        return Err(Error::Shape);
    }
    original_domain(domain, rows)?;
    budget::hiding(rows, 1)?;
    Ok(rows)
}

pub(crate) fn original_domain(domain: Domain, rows: usize) -> Result<Domain, Error> {
    if domain.size() != rows.checked_mul(2).ok_or(Error::Limits)? || domain.shift() == V::ZERO {
        return Err(Error::Shape);
    }
    let original = Domain::new(domain.shift(), rows.ilog2() as usize).ok_or(Error::Shape)?;
    let root = V::two_adic_generator(original.log_size() + 1);
    if root.square() != original.subgroup_generator() {
        return Err(Error::Shape);
    }
    Ok(original)
}

pub(crate) fn canonical_domain(coefficient_rows: usize) -> Result<Domain, Error> {
    if !coefficient_rows.is_power_of_two() {
        return Err(Error::Shape);
    }
    let log = coefficient_rows.ilog2() as usize + LOG_BLOWUP;
    Domain::new(V::GENERATOR, log).ok_or(Error::Shape)
}
