// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Challenger, CoefficientPcs, Dense, Domain, E, Error, V, budget, preflight};
use crate::compact::{LOG_BLOWUP, RANDOM_CODEWORDS};
use p3_commit::UnivariateStarkPcs;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::Matrix;

pub(super) fn ldes(
    pcs: &CoefficientPcs,
    evaluations: Vec<(Domain, Dense)>,
    chunks: usize,
) -> Result<Vec<Dense>, Error> {
    pcs.limits.validate()?;
    validate(pcs, &evaluations, chunks)?;
    Ok(UnivariateStarkPcs::<E, Challenger>::get_quotient_ldes(
        &pcs.stock,
        evaluations,
        chunks,
    )?)
}

fn validate(pcs: &CoefficientPcs, input: &[(Domain, Dense)], chunks: usize) -> Result<(), Error> {
    if !(2..=16).contains(&chunks) || !chunks.is_power_of_two() || input.len() != chunks {
        return Err(Error::Shape);
    }
    let shape = matrix_shape(&input[0].1)?;
    let mut shifts = Vec::with_capacity(chunks);
    for (domain, matrix) in input {
        check_matrix(*domain, matrix, shape)?;
        let power = domain.shift().exp_u64(shape.0 as u64);
        if shifts.contains(&power) {
            return Err(Error::Shape);
        }
        shifts.push(power);
    }
    capacity(pcs, shape, chunks)
}

fn check_matrix(domain: Domain, matrix: &Dense, shape: (usize, usize)) -> Result<(), Error> {
    if matrix.width() != shape.1
        || !matrix.values.len().is_multiple_of(shape.1)
        || matrix.height() != shape.0
        || domain.size() != shape.0
        || !(512..=65536).contains(&shape.0)
        || !shape.0.is_power_of_two()
        || domain.shift() == V::ZERO
    {
        return Err(Error::Shape);
    }
    budget::hiding(shape.0, 1)
}

fn matrix_shape(matrix: &Dense) -> Result<(usize, usize), Error> {
    let width = matrix.width();
    if width == 0 || !matrix.values.len().is_multiple_of(width) {
        return Err(Error::Shape);
    }
    Ok((matrix.height(), width))
}

fn capacity(pcs: &CoefficientPcs, shape: (usize, usize), chunks: usize) -> Result<(), Error> {
    let columns = shape.1.checked_add(RANDOM_CODEWORDS).ok_or(Error::Limits)?;
    let rows = shape.0.checked_mul(2).ok_or(Error::Limits)?;
    let mut total = (0usize, 0usize);
    for _ in 0..chunks {
        preflight::masked_shape(rows, columns, pcs.limits, &mut total)?;
    }
    budget::reserve(total.0, total.1, pcs.limits)?;
    let output = total
        .0
        .checked_shl(LOG_BLOWUP as u32)
        .ok_or(Error::Limits)?;
    if output > pcs.limits.max_evaluation_values {
        return Err(Error::Limits);
    }
    Ok(())
}
