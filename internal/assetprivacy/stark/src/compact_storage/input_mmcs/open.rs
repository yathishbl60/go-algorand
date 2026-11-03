// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    DenseData, InputError, InputMmcs, InputProverData, MultiProof, MultiValues, Storage, Streamed,
    Value32, gather,
};
use p3_commit::{BatchOpening, Mmcs};
use p3_matrix::Matrix;

impl InputMmcs {
    /// Fallible query validation; dense commits keep the stock's allocation behavior.
    pub fn try_open_multi<M: Matrix<Value32>>(
        &self,
        indices: &[usize],
        data: &InputProverData<M>,
    ) -> Result<(MultiValues, MultiProof), InputError> {
        match &data.storage {
            Storage::Dense(inner) => {
                self.validate_dense(indices, inner)?;
                Ok(self.stock.open_multi_batch(indices, inner))
            }
            Storage::Streamed(inner) => streamed_multi(indices, inner),
        }
    }

    pub fn try_open_batch<M: Matrix<Value32>>(
        &self,
        index: usize,
        data: &InputProverData<M>,
    ) -> Result<BatchOpening<Value32, Self>, InputError> {
        match &data.storage {
            Storage::Dense(inner) => {
                self.validate_dense(&[index], inner)?;
                let (values, proof) = self.stock.open_batch(index, inner).unpack();
                Ok(BatchOpening::new(values, proof))
            }
            Storage::Streamed(inner) => streamed_single(index, inner),
        }
    }

    fn validate_dense<M: Matrix<Value32>>(
        &self,
        indices: &[usize],
        data: &DenseData<M>,
    ) -> Result<(), InputError> {
        let matrices = self.stock.get_matrices(data);
        let height = matrices
            .iter()
            .map(|matrix| matrix.height())
            .max()
            .ok_or(InputError::Shape)?;
        if indices.iter().any(|index| *index >= height) {
            return Err(InputError::Shape);
        }
        gather::output_cells(matrices.iter().map(|m| m.width()), indices.len())?;
        Ok(())
    }
}

fn streamed_multi<M>(
    indices: &[usize],
    data: &Streamed<M>,
) -> Result<(MultiValues, MultiProof), InputError> {
    gather::validate(indices, data)?;
    let values = gather::rows(indices, data)?;
    let salts = gather::salts(indices, data)?;
    let proof = data
        .tree
        .pruned_path(indices)
        .ok_or(InputError::Allocation)?;
    Ok((values, (salts, proof)))
}

fn streamed_single<M>(
    index: usize,
    data: &Streamed<M>,
) -> Result<BatchOpening<Value32, InputMmcs>, InputError> {
    gather::validate(&[index], data)?;
    let mut values = gather::rows(&[index], data)?;
    let rows = values.pop().ok_or(InputError::Shape)?;
    let salts = data.tree.salts(index).ok_or(InputError::Allocation)?;
    let path = data.tree.path(index).ok_or(InputError::Allocation)?;
    Ok(BatchOpening::new(rows, (salts, path)))
}
