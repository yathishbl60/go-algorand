// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Arc, CoefficientInputData, CoefficientMatrix, InputError, InputMmcs, InputProverData,
    MaskedTrace, OwnedCoefficientMatrix, StreamedInputLimits, StreamedTree, TreeCap, Value32,
    gather,
};
use p3_dft::Radix2Dit;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_matrix::Matrix;

impl InputMmcs {
    /// Typed fast path only; generic Mmcs::commit always retains stock behavior.
    pub fn commit_coefficients(
        &self,
        coefficients: Vec<Arc<MaskedTrace<Value32>>>,
        domain: TwoAdicMultiplicativeCoset<Value32>,
        limits: StreamedInputLimits,
    ) -> Result<(TreeCap, CoefficientInputData), InputError> {
        let matrices = prepare(&coefficients, domain, limits)?;
        let tree = self.commit_tree(&coefficients, domain, limits)?;
        let cap = tree.cap().ok_or(InputError::Shape)?;
        let data = InputProverData::streamed(matrices, coefficients, domain, tree, limits);
        Ok((cap, data))
    }

    fn commit_tree(
        &self,
        coefficients: &[Arc<MaskedTrace<Value32>>],
        domain: TwoAdicMultiplicativeCoset<Value32>,
        limits: StreamedInputLimits,
    ) -> Result<StreamedTree, InputError> {
        let references: Vec<_> = coefficients.iter().map(Arc::as_ref).collect();
        let mut random = self.fork_random()?;
        StreamedTree::commit(
            &references,
            domain,
            limits.tile_columns,
            limits.max_tile_values,
            &Radix2Dit::default(),
            &mut random,
        )
        .ok_or(InputError::Allocation)
    }
}

fn prepare(
    coefficients: &[Arc<MaskedTrace<Value32>>],
    domain: TwoAdicMultiplicativeCoset<Value32>,
    limits: StreamedInputLimits,
) -> Result<Vec<OwnedCoefficientMatrix>, InputError> {
    limits.validate()?;
    if !valid_sources(coefficients) {
        return Err(InputError::Shape);
    }
    let mut matrices = Vec::new();
    matrices
        .try_reserve_exact(coefficients.len())
        .map_err(|_| InputError::Allocation)?;
    for source in coefficients {
        let matrix = CoefficientMatrix::new(Arc::clone(source), domain).ok_or(InputError::Shape)?;
        matrices.push(matrix.into_matrix().ok_or(InputError::Shape)?);
    }
    validate_budget(&matrices, limits)?;
    Ok(matrices)
}

fn valid_sources(coefficients: &[Arc<MaskedTrace<Value32>>]) -> bool {
    (1..=16).contains(&coefficients.len())
        && coefficients
            .iter()
            .all(|c| c.coefficient_rows() == coefficients[0].coefficient_rows())
}

fn validate_budget(
    matrices: &[OwnedCoefficientMatrix],
    limits: StreamedInputLimits,
) -> Result<(), InputError> {
    let width = gather::output_cells(matrices.iter().map(Matrix::width), 1)?;
    if width > limits.max_output_values {
        return Err(InputError::Limits);
    }
    for matrix in matrices {
        let tile = matrix
            .height()
            .checked_mul(limits.tile_columns.min(matrix.width()))
            .ok_or(InputError::Limits)?;
        if tile > limits.max_tile_values {
            return Err(InputError::Limits);
        }
    }
    Ok(())
}
