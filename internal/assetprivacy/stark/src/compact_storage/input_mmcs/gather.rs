// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientMatrix, InputError, MultiValues, Streamed};
use p3_dft::Radix2Dit;

pub(super) fn validate<M>(indices: &[usize], data: &Streamed<M>) -> Result<(), InputError> {
    if indices.iter().any(|index| *index >= data.tree.rows()) {
        return Err(InputError::Shape);
    }
    let output = output_cells(data.coefficients.iter().map(|c| c.width()), indices.len())?;
    if output > data.limits.max_output_values {
        return Err(InputError::Limits);
    }
    Ok(())
}

pub(super) fn output_cells(
    mut widths: impl Iterator<Item = usize>,
    queries: usize,
) -> Result<usize, InputError> {
    widths
        .try_fold(0usize, |n, width| n.checked_add(width))
        .and_then(|width| width.checked_mul(queries))
        .ok_or(InputError::Limits)
}

pub(super) fn rows<M>(indices: &[usize], data: &Streamed<M>) -> Result<MultiValues, InputError> {
    let mut output = allocate(indices.len(), data.coefficients.len())?;
    for coefficients in &data.coefficients {
        let source =
            CoefficientMatrix::new(coefficients.as_ref(), data.domain).ok_or(InputError::Shape)?;
        let rows = source
            .gather(
                indices,
                data.limits.tile_columns,
                data.limits.max_tile_values,
                data.limits.max_output_values,
                &Radix2Dit::default(),
            )
            .ok_or(InputError::Allocation)?;
        for (query, row) in output.iter_mut().zip(rows) {
            query.push(row);
        }
    }
    Ok(output)
}

fn allocate(queries: usize, matrices: usize) -> Result<MultiValues, InputError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(queries)
        .map_err(|_| InputError::Allocation)?;
    for _ in 0..queries {
        let mut row = Vec::new();
        row.try_reserve_exact(matrices)
            .map_err(|_| InputError::Allocation)?;
        output.push(row);
    }
    Ok(output)
}

pub(super) fn salts<M>(indices: &[usize], data: &Streamed<M>) -> Result<MultiValues, InputError> {
    let mut salts = Vec::new();
    salts
        .try_reserve_exact(indices.len())
        .map_err(|_| InputError::Allocation)?;
    for index in indices {
        salts.push(data.tree.salts(*index).ok_or(InputError::Allocation)?);
    }
    Ok(salts)
}
