// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{InputMmcs, InputProverData, MultiProof, Proof, Stock, Storage, Value32 as V};
use p3_commit::{BatchOpening, BatchOpeningRef, Mmcs, MmcsProofGeometry};
use p3_matrix::{Dimensions, Matrix};

impl Mmcs<V> for InputMmcs {
    type ProverData<M> = InputProverData<M>;
    type Commitment = <Stock as Mmcs<V>>::Commitment;
    type Proof = Proof;
    type MultiProof = MultiProof;
    type Error = <Stock as Mmcs<V>>::Error;

    fn proof_geometry(&self, log: usize) -> Option<MmcsProofGeometry> {
        self.stock.proof_geometry(log)
    }

    fn commit<M: Matrix<V>>(&self, inputs: Vec<M>) -> (Self::Commitment, Self::ProverData<M>) {
        let (cap, data) = self.stock.commit(inputs);
        (cap, InputProverData::dense(data))
    }

    fn open_batch<M: Matrix<V>>(
        &self,
        index: usize,
        data: &Self::ProverData<M>,
    ) -> BatchOpening<V, Self> {
        self.try_open_batch(index, data)
            .expect("invalid input-MMCS single-opening request")
    }

    fn get_matrices<'a, M: Matrix<V>>(&self, data: &'a Self::ProverData<M>) -> Vec<&'a M> {
        match &data.storage {
            Storage::Dense(inner) => self.stock.get_matrices(inner),
            Storage::Streamed(inner) => inner.matrices.iter().collect(),
        }
    }

    fn verify_batch(
        &self,
        cap: &Self::Commitment,
        dimensions: &[Dimensions],
        index: usize,
        opening: BatchOpeningRef<'_, V, Self>,
    ) -> Result<(), Self::Error> {
        self.stock.verify_batch(
            cap,
            dimensions,
            index,
            BatchOpeningRef::new(opening.opened_values, opening.opening_proof),
        )
    }

    fn open_multi_batch<M: Matrix<V>>(
        &self,
        indices: &[usize],
        data: &Self::ProverData<M>,
    ) -> (Vec<Vec<Vec<V>>>, Self::MultiProof) {
        self.try_open_multi(indices, data)
            .expect("invalid input-MMCS multi-opening request")
    }

    fn verify_multi_batch<R: AsRef<[V]> + PartialEq>(
        &self,
        cap: &Self::Commitment,
        dimensions: &[Dimensions],
        indices: &[usize],
        values: &[Vec<R>],
        proof: &Self::MultiProof,
    ) -> Result<(), Self::Error> {
        self.stock
            .verify_multi_batch(cap, dimensions, indices, values, proof)
    }
}
