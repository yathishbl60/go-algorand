// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Domain, E, Error, V, zeroes};
use crate::compact_storage::MaskedTrace;
use p3_dft::Radix2Dit;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn compress(
    pcs: &CoefficientPcs,
    source: &MaskedTrace<V>,
    domain: Domain,
    alpha: E,
    offset: E,
) -> Result<Vec<E>, Error> {
    let mut compressed = zeroes(domain.size())?;
    let dft = Radix2Dit::<V>::default();
    let tile_columns = pcs.limits.tile_columns;
    for start in (0..source.width()).step_by(tile_columns) {
        let end = source.width().min(start.saturating_add(tile_columns));
        let tile = source
            .evaluate_columns(start..end, domain, pcs.limits.max_tile_values, &dft)
            .ok_or(Error::Limits)?;
        let weights = weights(end - start, alpha, offset * alpha.exp_u64(start as u64))?;
        accumulate(&tile, &mut compressed, domain.log_size(), &weights);
    }
    Ok(compressed)
}

fn weights(width: usize, alpha: E, first: E) -> Result<Vec<E>, Error> {
    let mut weights = zeroes(width)?;
    let mut power = first;
    for weight in &mut weights {
        *weight = power;
        power *= alpha;
    }
    Ok(weights)
}

fn accumulate(tile: &RowMajorMatrix<V>, output: &mut [E], log: usize, weights: &[E]) {
    for (index, row) in tile.values.chunks_exact(tile.width).enumerate() {
        let mut value = E::ZERO;
        for (&cell, &weight) in row.iter().zip(weights) {
            value += weight * cell;
        }
        output[index.reverse_bits() >> (usize::BITS as usize - log)] += value;
    }
}
