// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{E, Error, Request, V, zeroes};
use crate::compact_storage::MaskedTrace;
use p3_commit::OpenedValues;
use p3_field::PrimeCharacteristicRing;

pub(super) fn evaluate(rounds: &[Request<'_>]) -> Result<OpenedValues<E>, Error> {
    rounds
        .iter()
        .map(|round| {
            round
                .prover_data
                .inner
                .sources
                .iter()
                .zip(&round.points)
                .map(|(source, points)| {
                    points
                        .iter()
                        .map(|&point| evaluate_point(source, point))
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn evaluate_point(source: &MaskedTrace<V>, point: E) -> Result<Vec<E>, Error> {
    let mut values = zeroes(source.width())?;
    let mut power = E::ONE;
    for row in source.coefficients().values.chunks_exact(source.width()) {
        for (value, &coefficient) in values.iter_mut().zip(row) {
            *value += power * coefficient;
        }
        power *= point;
    }
    Ok(values)
}

pub(super) fn split(rounds: &[Request<'_>], values: &mut OpenedValues<E>) -> OpenedValues<E> {
    rounds
        .iter()
        .zip(values)
        .map(|(round, batch)| split_batch(&round.prover_data.inner.widths, batch))
        .collect()
}

fn split_batch(widths: &[usize], batch: &mut [Vec<Vec<E>>]) -> Vec<Vec<Vec<E>>> {
    widths
        .iter()
        .zip(batch)
        .map(|(&width, points)| {
            points
                .iter_mut()
                .map(|columns| columns.split_off(width))
                .collect()
        })
        .collect()
}
