// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::state::CommitmentData;
use super::{CoefficientPcs, Domain, Error, Request, V};
use crate::compact::{
    LOG_BLOWUP, LOG_FINAL_POLY_LEN, MAX_LOG_ARITY, NUM_QUERIES, RANDOM_CODEWORDS,
};
use crate::compact_storage::MaskedTrace;
use p3_commit::Mmcs;
use p3_field::{ExtensionField, Field};
use p3_matrix::Matrix;

pub(super) fn parameters(pcs: &CoefficientPcs) -> Result<(), Error> {
    let fri = &pcs.fri;
    if fri.log_blowup != LOG_BLOWUP
        || fri.log_final_poly_len != LOG_FINAL_POLY_LEN
        || fri.max_log_arity != MAX_LOG_ARITY
        || fri.num_queries != NUM_QUERIES
    {
        return Err(Error::Unsupported);
    }
    Ok(())
}

pub(super) fn round(pcs: &CoefficientPcs, round: &Request<'_>) -> Result<(), Error> {
    let data = &round.prover_data.inner;
    shape(data, round.points.len())?;
    let matrices = pcs.input.get_matrices(&data.input);
    if matrices.len() != data.sources.len() {
        return Err(Error::Shape);
    }
    for (index, committed) in matrices.into_iter().enumerate() {
        let dimensions = (committed.width(), committed.height());
        matrix(data, index, dimensions, &round.points[index])?;
    }
    Ok(())
}

fn shape(data: &CommitmentData, count: usize) -> Result<(), Error> {
    if data.preprocessed
        || !(1..=16).contains(&data.sources.len())
        || data.sources.len() != count
        || count != data.widths.len()
    {
        return Err(Error::Shape);
    }
    Ok(())
}

fn matrix(
    data: &CommitmentData,
    index: usize,
    dimensions: (usize, usize),
    points: &[super::E],
) -> Result<(), Error> {
    let source = &data.sources[index];
    if dimensions != (source.width(), data.lde_domain.size())
        || data.widths[index].checked_add(RANDOM_CODEWORDS) != Some(source.width())
    {
        return Err(Error::Shape);
    }
    matrix_points(source, data.lde_domain, points)
}

fn matrix_points(
    source: &MaskedTrace<V>,
    domain: Domain,
    points: &[super::E],
) -> Result<(), Error> {
    if !trace_geometry(source, domain) || points.is_empty() {
        return Err(Error::Shape);
    }
    if points
        .iter()
        .any(|p| p.as_base().is_some_and(|x| domain.contains(x)))
    {
        return Err(Error::Shape);
    }
    super::super::budget::hiding(source.trace_rows(), points.len())
}

fn trace_geometry(source: &MaskedTrace<V>, domain: Domain) -> bool {
    let rows = source.trace_rows();
    (512..=65536).contains(&rows)
        && rows.is_power_of_two()
        && source.coefficient_rows() == rows * 2
        && source.relation_width() > 0
        && domain.size() == rows * 2 * (1 << LOG_BLOWUP)
        && domain.shift() == V::GENERATOR
}
