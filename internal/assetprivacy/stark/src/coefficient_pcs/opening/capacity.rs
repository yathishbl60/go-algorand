// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Error, Request};
use crate::compact::{NUM_QUERIES, RANDOM_CODEWORDS};

pub(super) struct Counts {
    heights: [bool; 33],
    claims: usize,
    widths: usize,
}
impl Counts {
    pub(super) fn new() -> Self {
        Self {
            heights: [false; 33],
            claims: 0,
            widths: 0,
        }
    }
}

pub(super) fn count(
    pcs: &CoefficientPcs,
    round: &Request<'_>,
    counts: &mut Counts,
) -> Result<(), Error> {
    let data = &round.prover_data.inner;
    let height = data.lde_domain.size();
    counts.heights[data.lde_domain.log_size()] = true;
    for (width, points) in data.widths.iter().zip(&round.points) {
        let width = width.checked_add(RANDOM_CODEWORDS).ok_or(Error::Limits)?;
        counts.widths = counts.widths.checked_add(width).ok_or(Error::Limits)?;
        let claims = width.checked_mul(points.len()).ok_or(Error::Limits)?;
        counts.claims = counts.claims.checked_add(claims).ok_or(Error::Limits)?;
        tile(pcs, height, width)?;
    }
    Ok(())
}

pub(super) fn buffers(pcs: &CoefficientPcs, counts: &Counts) -> Result<usize, Error> {
    let heights = &counts.heights;
    let max_log = heights.iter().rposition(|&used| used).ok_or(Error::Shape)?;
    let total = heights
        .iter()
        .enumerate()
        .filter(|(_, used)| **used)
        .try_fold(0usize, |sum, (log, _)| sum.checked_add(1 << log))
        .ok_or(Error::Limits)?;
    let peak = total
        .checked_add((1usize << max_log).checked_mul(3).ok_or(Error::Limits)?)
        .ok_or(Error::Limits)?;
    cap(peak, pcs.limits.max_evaluation_values, 10)?;
    cap(counts.claims, pcs.limits.max_claim_values, 10)?;
    cap(counts.widths, pcs.limits.max_query_values, NUM_QUERIES)?;
    Ok(max_log)
}

fn cap(value: usize, limit: usize, scale: usize) -> Result<(), Error> {
    let value = value.checked_mul(scale).ok_or(Error::Limits)?;
    if value > limit {
        return Err(Error::Limits);
    }
    Ok(())
}

fn tile(pcs: &CoefficientPcs, height: usize, width: usize) -> Result<(), Error> {
    cap(
        width.min(pcs.limits.tile_columns),
        pcs.limits.max_evaluation_values,
        10,
    )?;
    let cells = height
        .checked_mul(width.min(pcs.limits.tile_columns))
        .ok_or(Error::Limits)?;
    cap(cells, pcs.limits.max_tile_values, 1)
}
